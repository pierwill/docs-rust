use bson::Document;
use futures::TryStreamExt;
use mongodb::{ bson::doc, Client, Collection, options::{ FindOptions } };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Document> = client.database("db").collection("fruits");
    // Sample documents
    let docs = vec! [
      //begin-sample-docs
       doc! { "_id": 1, "name": "orange", "quantity": 7 },
       doc! { "_id": 2, "name": "apple", "quantity": 4, "description": "Granny Smith" },
       doc! { "_id": 3, "name": "banana", "quantity": 36},
       doc! { "_id": 4, "name": "pear", "quantity": 28, "vendors": ["A", "C" ] }
       //end-sample-docs
    ];
    // Inserts sample documents into the collection
    my_coll.insert_many(docs, None).await?;

    //begin-literal
    let query = doc! { "name": "pear" };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    } 
    //end-literal
    println!("");
    //begin-comparison
    // $gt means "greater than"
    let query = doc! { "quantity": doc! { "$gt": 5 } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-comparison
    println!("");
    //begin-logical
    let query = doc! { "$and": vec! [
           doc! { "quantity": doc! { "$gt": 10 } },
           doc! { "quantity": doc! { "$mod": [ 3, 0 ] } }
       ]
    };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-logical
    println!("");
    //begin-element
    let query = doc! { "description": doc! { "$exists": true } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-element
    println!("");
    //begin-evaluation
    // $mod means "modulo" and checks if the remainder is a specific value
    let query = doc! { "quantity": doc! { "$mod": [ 3, 0 ] } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
       let doc: Document = bson::from_document(result)?;
       println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-evaluation
    println!("");
    //begin-bitwise
    let query = doc! { "quantity": doc! { "$bitsAllSet": 7 } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
        let doc: Document = bson::from_document(result)?;
        println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-bitwise
    println!("");
    //begin-array
    let query = doc! { "vendors": doc! { "$elemMatch": { "$eq": "C" } } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
       let doc: Document = bson::from_document(result)?;
       println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-array

    Ok(())
}