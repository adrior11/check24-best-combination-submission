#[macro_export]
macro_rules! populate_from_csv {
    ($db:expr, $coll:expr, $model:ty, $csv_path:expr) => {
        let collection: mongodb::Collection<$model> = $db.collection($coll); 

        let count = collection.estimated_document_count().await?;
        if count == 0 {
            let file = std::fs::File::open($csv_path)?;
            dbg!(&file);
            let mut rdr = csv::Reader::from_reader(file);
            dbg!(&rdr);

            let mut counter = 0;
            for result in rdr.records() {
                dbg!(&result);
                dbg!(&counter);
                println!("File {}", $csv_path);
                // let r: $model = result?;
                // println!("{:?}", r);
                let record = result?;
                dbg!(&record);
                // collection.insert_one(record).await?;
                counter+=1;
                if counter == 10 { break; }
            }
        }
    };
}
