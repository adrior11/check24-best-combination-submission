FROM mongo:latest

RUN mkdir -p /app/csv_data

WORKDIR /app/csv_data

COPY bc_game.csv /app/csv_data/

COPY bc_streaming_offer.csv /app/csv_data/

COPY bc_streaming_package.csv /app/csv_data/

EXPOSE 27017

CMD ["bash", "-c", "mongod --bind_ip_all --fork --logpath /var/log/mongodb.log && sleep 5 && if [ $(mongo --quiet --eval 'db.getCollectionNames().length' best_combination) -eq 0 ]; then mongoimport --type csv -d best_combination -c bc_game --file /app/csv_data/bc_game.csv --headerline && mongoimport --type csv -d best_combination -c bc_streaming_offer --file /app/csv_data/bc_streaming_offer.csv --headerline && mongoimport --type csv -d best_combination -c bc_streaming_package --file /app/csv_data/bc_streaming_package.csv --headerline; else echo 'Database already populated, skipping import.'; fi && tail -f /var/log/mongodb.log"]
