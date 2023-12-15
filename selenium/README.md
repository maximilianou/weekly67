

https://blog.logrocket.com/testing-website-selenium-docker/

nvm

https://github.com/nvm-sh/nvm

```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

nvm install node

npm init -y

```



```js
// selenium/chrome.test.js
const webdriver = require('selenium-webdriver');
const { Builder, Capabilities } = webdriver
let capabilities = Capabilities.chrome();

describe("Test if www.debian.org's home page's title is correct", () => {
  let driver;

  beforeAll(async () => {
      driver = new Builder()
          .usingServer('http://localhost:4444')
          .withCapabilities(capabilities)
          .build();
      await driver.get("https://www.debian.org/");
  }, 70000);

  afterAll(async () => {
      await driver.quit();
  }, 120000);

  it('test', async () => {
    try {
        let title = (await driver.getTitle()).trim();
        expect(title).toEqual("Debian -- The Universal Operating System");
    } catch (err) {
        throw err;
    }
  }, 35000);

});

```

https://jestjs.io/

```sh
npm install jest
```

- package.json
```json
{ 
  "name": "selenium",
  "version": "1.0.0",
  "description": "https://blog.logrocket.com/testing-website-selenium-docker/",
  "main": "chrome.test.js",
  "scripts": {
    "test": "jest "
  },
  "author": "",
  "license": "ISC",
  "dependencies": {
    "jest": "^29.7.0",
    "selenium-webdriver": "^4.16.0"
  }
}

```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/selenium]
└─$ npm test        

> selenium@1.0.0 test
> jest

 FAIL  ./chrome.test.js
  ● Test suite failed to run

    Cannot find module 'selenium-webdriver' from 'chrome.test.js'

    > 1 | const webdriver = require('selenium-webdriver');
```

```sh
npm install selenium-webdriver
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/selenium]
└─$ npm test                      

> selenium@1.0.0 test
> jest


 RUNS  ./chrome.test.js
node:internal/process/promises:289
            triggerUncaughtException(err, true /* fromPromise */);
            ^

Error: ECONNREFUSED connect ECONNREFUSED 127.0.0.1:4444
```

```sh
# commandline Ctrl+C
docker run -p 4444:4444 -p 7900:7900 --shm-size="2g" selenium/standalone-chrome

# detached
docker run -d -p 4444:4444 -p 7900:7900 --shm-size="2g" selenium/standalone-chrome
```


```sh
┌──(kali㉿kali)-[~/projects/weekly67/selenium]
└─$ npm test

> selenium@1.0.0 test
> jest

 PASS  ./chrome.test.js (15.947 s)
  Test if www.debian.org's home page's title is correct
    ✓ test (975 ms)

Test Suites: 1 passed, 1 total
Tests:       1 passed, 1 total
Snapshots:   0 total
Time:        16.34 s, estimated 17 s
Ran all test suites.
```

- Browser - noVnc
```
http://localhost:7900/
```







