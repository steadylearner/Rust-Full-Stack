// https://github.com/stevepryde/thirtyfour/issues/11

use thirtyfour::sync::prelude::*;

fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
  
     // docker run --rm -d -p 4444:4444 -p 5900:5900 --name selenium-server -v /dev/shm:/dev/shm selenium/standalone-chrome-debug:3.141.59-zinc
     let docker_selenium = "http://localhost:4444/wd/hub";

     let driver = WebDriver::new(docker_selenium, &caps)?;

     // Navigate to https://wikipedia.org.
     driver.get("https://wikipedia.org")?;
     let elem_form = driver.find_element(By::Id("search-form"))?;

     // Find element from element.
     let elem_text = elem_form.find_element(By::Id("searchInput"))?;

     // Type in the search terms.
     elem_text.send_keys("selenium")?;

     // Click the search button.
     let elem_button = elem_form.find_element(By::Css("button[type='submit']"))?;
     elem_button.click()?;

     // Look for header to implicitly wait for the page to load.
     driver.find_element(By::ClassName("firstHeading"))?;
     let title = driver.title()?;
     println!("{}", title);

     Ok(())
}
