use crate::schema::products;

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use crate::schema::products::dsl;
use crate::schema::products::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>,
}

// Use product_id instead of id not to make confusion for compiler for duplicate variable names.
impl Product {
    pub fn find(product_id: &i32, connection: &PgConnection) -> Result<Product, diesel::result::Error> {
        products::table.find(product_id).first(connection)
    }

    pub fn destroy(product_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::products.find(product_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(product_id: &i32, new_product: &NewProduct, connection: &PgConnection) -> Result<(), diesel::result::Error> {

        diesel::update(dsl::products.find(product_id))
            .set(new_product)
            .execute(connection)?;
        Ok(())
    }
}

//  Move this methods to function or wihout them.
#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list(connection: &PgConnection) -> Self {

        let result =
            products
                .limit(10)
                .load::<Product>(connection)
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
    pub fn create(&self, connection: &PgConnection) -> Result<Product, diesel::result::Error> {

        diesel::insert_into(products::table)
            .values(self)
            .get_result(connection)
    }
}


