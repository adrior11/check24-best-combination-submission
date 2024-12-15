#!/bin/bash

mongod --bind_ip_all --logpath /var/log/mongodb.log &

until mongosh --quiet --eval "db.adminCommand('ping')" >/dev/null 2>&1; do
    echo "Waiting for MongoDB to be ready..."
    sleep 2
done

# Create the root/admin user
if [[ -n "$MONGO_INITDB_ROOT_USERNAME" && -n "$MONGO_INITDB_ROOT_PASSWORD" ]]; then
    echo "Creating root user..."
    mongosh admin --eval "db.createUser({user: '$MONGO_INITDB_ROOT_USERNAME', pwd: '$MONGO_INITDB_ROOT_PASSWORD', roles: [{ role: 'userAdminAnyDatabase', db: 'admin' }, { role: 'root', db: 'admin' }]});"
fi

# Import data into the best_combination database if not already present
check_and_import() {
    local db=$1
    local collection=$2
    local file=$3

    # Check if the collection has any documents
    count=$(mongosh --quiet --eval "db.getSiblingDB('$db').$collection.countDocuments()" || echo "error")

    # Import if the collection is empty
    if [ "$count" = "error" ] || ! [[ "$count" =~ ^[0-9]+$ ]]; then
        echo "Error checking collection $db.$collection count."
    elif [ "$count" -eq 0 ]; then
        echo "Importing data into $db.$collection from $file..."
        mongoimport --type csv -d "$db" -c "$collection" --file "$file" --headerline
    else
        echo "Collection $db.$collection already has data, skipping import."
    fi
}

check_and_import "best_combination" "bc_game" "/app/csv_data/bc_game.csv"
check_and_import "best_combination" "bc_streaming_offer" "/app/csv_data/bc_streaming_offer.csv"
check_and_import "best_combination" "bc_streaming_package" "/app/csv_data/bc_streaming_package.csv"

# Create database indexes
mongosh --quiet <<EOF
use best_combination;

function ensureIndex(collectionName, indexSpec, indexName) {
    const collection = db.getCollection(collectionName);
    const indexes = collection.getIndexes().map(i => i.name);
    if (!indexes.includes(indexName)) {
        print(\`Creating index for \${collectionName}...\`);
        collection.createIndex(indexSpec, { name: indexName });
    } else {
        print(\`Index \${indexName} already exists for \${collectionName}.\`);
    }
}

ensureIndex("bc_game", { team_home: 1, team_away: 1 }, "team_home_1_team_away_1");
ensureIndex("bc_streaming_offer", { game_id: 1 }, "game_id_1");
ensureIndex("bc_streaming_offer", { streaming_package_id: 1 }, "streaming_package_id_1");
ensureIndex("bc_streaming_package", { streaming_package_id: 1 }, "streaming_package_id_1");
EOF

# Keep MongoDB running in the foreground
tail -f /var/log/mongodb.log

