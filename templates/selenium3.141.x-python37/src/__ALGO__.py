import Algorithmia
from selenium import webdriver
from selenium.webdriver.common.keys import Keys

def get_chrome():
    # Chromium must be launched in headless mode
    chrome_options = webdriver.ChromeOptions()
    chrome_options.add_argument('--no-sandbox')
    chrome_options.add_argument('--window-size=1420,1080')
    chrome_options.add_argument('--headless')
    chrome_options.add_argument('--disable-gpu')
    driver = webdriver.Chrome(chrome_options=chrome_options)
    return driver

def get_phantomjs():
    # PhantomJS must put logs in a place where it has write permissions
    driver = webdriver.PhantomJS(service_log_path="/tmp/ghostdriver.log")
    return driver

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def apply(input):
    # note: you MUST have internet access enabled for this test work
    driver = get_chrome()
    #driver = get_phantomjs()
    driver.get("http://www.python.org")
    if 'Python' not in driver.title:
        return {"status": "failure",
            "message": "failed to load page"}
    elem = driver.find_element_by_name("q")
    elem.clear()
    elem.send_keys("pycon")
    elem.send_keys(Keys.RETURN)
    if 'No results found' in driver.page_source:
        return {"status": "failure",
            "message": "Failed to find required text in page body"}
    driver.close()
    return {"status": "success"}
