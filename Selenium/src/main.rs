// https://github.com/stevepryde/thirtyfour
// When you use its async API, you should not forget to use "await?"

// 1. Setup Selenium with Docker before you test this code.
// $docker run --rm -d -p 4444:4444 -p 5900:5900 --name selenium-server -v /dev/shm:/dev/shm selenium/standalone-chrome-debug:3.141.59-zinc

// 2. When you verify it work, $docker stop selenium-server 
// to save the resource of your machine.

// The machine used here is very weak. Hope your result will be better.
// $time cargo run --release
// real	0m47.384s
// user	1m29.830s
// sys	0m1.701s

// Size: 6.6 MB

// use std::fs::{write};

use thirtyfour::error::WebDriverResult;
use thirtyfour::{
    By, 
    DesiredCapabilities, 
    WebDriver,
    Keys,
    TypingData,
};

use tokio;

// You should have investigated https://www.steadylearner.com/blog/search/Rust
// With CTRL + SHIFT + I
// Then, find the workflow to make Selenium work.

// Result<(), Box<dyn std::error::Error>>
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await?;

    let target = "https://www.steadylearner.com";
    println!("Wait before you play with {}\n", &target);

    // Navigate to https://www.steadylearner.com.
    driver.get(target).await?;
    // println!("{:#?}", &driver.title().await?); // Should be "Steadylearner Home"

    // 1. Click "search" icon to show input bar first.
    // <span class="hover" title="You can use [Ctrl + / ] to start search."><span class="font-two margin-right-half">Search</span><i class="fa fa-search search-icon"></i></span>

    let search_icon = driver.find_element(By::Css("span[title='You can use [Ctrl + / ] to start search.']")).await?; 
    search_icon.click().await?;

    // 2. Find "input" bar to type "Rust" and "Enter" to Submit
    let search_input = driver.find_element(By::Id("searchInput")).await?;
    // println!("{}", &search_input.get_attribute("value").await?); // ""

    // search_input.send_keys("Rust").await?;
    // search_input.send_keys(Keys::Enter).await?;
    // Refer to https://docs.rs/thirtyfour/0.4.1/thirtyfour/struct.WebElement.html#method.send_keys
    
    search_input.send_keys(TypingData::from("Rust") + Keys::Enter).await?;
    // println!("{:#?}", &driver.title().await?); // "Posts for Rust"

    let class_to_get_blog_titles = "search-result-link__title";
    let blog_titles = driver.find_elements(By::ClassName(&class_to_get_blog_titles)).await?;
    // println!("{:#?}", blog_titles);

    // https://stackoverflow.com/questions/6003819/what-is-the-difference-between-properties-and-attributes-in-html
    // https://stackoverflow.com/questions/43627340/what-is-the-difference-between-property-and-attribute-in-selenium-webelement
    // https://docs.rs/thirtyfour/0.4.1/thirtyfour/struct.WebElement.html#method.text
    
    // for (i, blog_title) in blog_titles.enumerate() {
    // method not found in `std::vec::Vec<thirtyfour::webelement::WebElement>`
    //  |
    //  = note: the method `enumerate` exists but the following trait bounds were not satisfied:
    //      `&mut [thirtyfour::webelement::WebElement] : std::iter::Iterator`
    //      `&mut std::vec::Vec<thirtyfour::webelement::WebElement> : std::iter::Iterator`

    // Not Rust way, but it works.
    // https://github.com/rust-lang/rust-clippy/issues/159
    //  let mut index = 0;
    //  for blog_title in blog_titles {
    //      index = index + 1;
    //      let title = blog_title.text().await?;
    //      let link = title.replace(" ", "-");
    //      // println!("{}", &title);
    //      // println!("{}{}", &target, &link);
    //      println!("{}. [{}]({}{})", &num, &title, &target, &link);
    //  }

    let mut contents = String::new();

    for (index, blog_title) in blog_titles.iter().enumerate() {
        let number = index + 1;
        let title = blog_title.text().await?;
        let link = title.replace(" ", "-");
        let md = format!("{}. [{}]({}{})\n", &number, &title, &target, &link);
        print!("{}", &md);
        contents.push_str(&md);
    }

    let file_name = "the_latest_rust_blog_posts_from_steadylearner.md";
    // [sync]
    // write(&file_name, &contents)?;
    
    // [async] - should use this in async run time not to block.
    tokio::fs::write(&file_name, &contents).await?;
    println!("\nCould build {} with Rust thirtyfour crate to use Selenium.", &file_name);
    
    Ok(())
}
