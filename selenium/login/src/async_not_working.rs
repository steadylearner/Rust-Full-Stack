use thirtyfour::prelude::*;
use tokio;

// Xpaths

// Username, //*[@id="loginUsername"]
// Password, //*[@id="loginPassword"]
// Sign In, /html/body/div/div/div[2]/div/form/div/fieldset[5]/button

// let username = driver.find_element(By::Id("search-form")).await?;
// // Type username
// let password = driver.find_element(By::Id("search-form")).await?;
// // Type password
// let signin = driver.find_element(By::Id("search-form")).await?;
// // Click

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444", &caps).await?;

     driver.get("https://www.reddit.com/login").await?;
     // let elem_form = driver.find_element(By::Id("search-form")).await?;

     // // Find element from element.
     // let elem_text = elem_form.find_element(By::Id("searchInput")).await?;

     // // Type in the search terms.
     // elem_text.send_keys("selenium").await?;

     // // Click the search button.
     // let elem_button = elem_form.find_element(By::Css("button[type='submit']")).await?;
     // elem_button.click().await?;

     // // Look for header to implicitly wait for the page to load.
     // driver.find_element(By::ClassName("firstHeading")).await?;
     // assert_eq!(driver.title().await?, "Selenium - Wikipedia");

     Ok(())
}

// Error: ReqwestError(reqwest::Error { kind: Decode, source: Error("expected value", line: 1, column: 1) })
// [Finished running. Exit status: 1]
