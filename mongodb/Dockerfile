FROM mongo:latest

RUN mkdir -p /app/csv_data

WORKDIR /app/csv_data

COPY bc_game.csv /app/csv_data/

COPY bc_streaming_offer.csv /app/csv_data/

COPY bc_streaming_package.csv /app/csv_data/

COPY mongo_init.sh /app/mongo_init.sh

RUN chmod +x /app/mongo_init.sh

CMD ["bash", "/app/mongo_init.sh"]
