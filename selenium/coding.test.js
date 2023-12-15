// selenium/chrome.test.js
const webdriver = require('selenium-webdriver');
const { Builder, Capabilities, By, until } = webdriver
let capabilities = Capabilities.chrome();
/*
describe("Test if codepen.io's home page's title is correct", () => {
  let driver;

  beforeAll(async () => {
      driver = new Builder()
          .usingServer('http://localhost:4444')
          .withCapabilities(capabilities)
          .build();
        //  await driver.get("https://codepen.io/");
          await driver.get("https://codepen.io/pen");
        }, 70000);

  afterAll(async () => {
      await driver.quit();
  }, 80000);

  it('test', async () => {
    try {
//        let title = (await driver.getTitle()).trim();
//        expect(title).toEqual("CodePen: Online Code Editor and Front End Web Developer Community");
//        await driver.findElement(By.xpath("/html/body/div[1]/div/header/div/nav/div[1]/a")).click();

//        await driver.wait(until.titleIs("Create a New Pen"), 15000);

        let inputHTML = await driver.findElement(By.xpath("/html/body/div[2]/div/div[2]/div[2]/div[2]/div[1]/div[6]"));
        inputHTML.sendKeys("<a id='thelink' href='codepen.io'>This Link</a>");

//        await driver.manage().setTimeouts({ implicit: 5000 });

//        let theLinkText = await driver.findElement(By.id("thelink"));

//        await driver.wait(until.elementLocated(By.id('thelink')), 30000);

        let theLinkText = (driver.findElement(By.xpath("//*[@id='thelink']")));
        
        expect(theLinkText).toEqual("This Link");


    } catch (err) {
        throw err;
    }
  }, 35000);

});
*/
