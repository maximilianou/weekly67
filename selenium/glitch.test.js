// selenium/chrome.test.js
const webdriver = require('selenium-webdriver');
const { Builder, Capabilities } = webdriver
let capabilities = Capabilities.chrome();

describe("Test if glitch.com's home page's title is correct", () => {
  let driver;

  beforeAll(async () => {
      driver = new Builder()
          .usingServer('http://localhost:4444')
          .withCapabilities(capabilities)
          .build();
      await driver.get("https://glitch.com/");
  }, 70000);

  afterAll(async () => {
      await driver.quit();
  }, 120000);

  it('test', async () => {
    try {
        let title = (await driver.getTitle()).trim();
        expect(title).toEqual("Glitch: The friendly community where everyone builds the web");
    } catch (err) {
        throw err;
    }
  }, 35000);

});

