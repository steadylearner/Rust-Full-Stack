use crate::schema::products;

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use crate::schema::products::dsl;
use crate::schema::products::dsl::*;
use crate::db_connection::establish_connection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>,
}

// Use product_id instead of id not to make confusion for compiler for duplicate variable names.
impl Product {
    pub fn find(product_id: &i32) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();
        products::table.find(product_id).first(&connection)
    }

    pub fn destroy(product_id: &i32) -> Result<(), diesel::result::Error> {
        let connection = establish_connection();
        diesel::delete(dsl::products.find(product_id)).execute(&connection)?;
        Ok(())
    }

    pub fn update(product_id: &i32, new_product: &NewProduct) -> Result<(), diesel::result::Error> {
        let connection = establish_connection();

        diesel::update(dsl::products.find(product_id))
            .set(new_product)
            .execute(&connection)?;
        Ok(())
    }
}

//  Move this methods to function or wihout them.
#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list() -> Self {
        let connection = establish_connection();

        let result =
            products
                .limit(10)
                .load::<Product>(&connection)
                .expect("Error loading products");

        ProductList(result)
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}

impl NewProduct {
    pub fn create(&self) -> Result<Product, diesel::result::Error> {

        let connection = establish_connection();
        diesel::insert_into(products::table)
            .values(self)
            .get_result(&connection)
    }
}


