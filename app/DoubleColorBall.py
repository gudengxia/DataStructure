from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.common.by import By
from bs4 import BeautifulSoup
import time

#https://www.cwl.gov.cn/ygkj/wqkjgg/ssq/
total_page = 61
class DoubleColorBall(object):
    def __init__(self):
        #self.balls = []
        self.base_url = "https://www.cwl.gov.cn/ygkj/wqkjgg/ssq/"
        self.data_file = "./dcb.txt"
        self.driver = webdriver.Firefox()
        self.f = open(self.data_file, 'w+')

    def get_balls(self, html):
        soup = BeautifulSoup(html, "html.parser")
        table = soup.find('table', class_ = 'ssq_table')
        if not table:
            print('table not found')
            return

        balls = []
        for row in table.find('tbody').find_all('tr'):
            win = []
            win.append(row.find('td').text)
            for ball in row.find('div', class_ = 'qiu').find_all('div', class_= 'qiu-item'):
                win.append(ball.text)
            balls.append(win)

        for row in balls:
            for item in row:
                self.f.write(item + '\t')
            self.f.write('\n')
    def run(self):
        self.driver.get(self.base_url)
        html = self.driver.page_source
        print("Begin extract data in page 1.")
        for page_no in range(2, total_page+1):
            print("Begin extract data in page {}.".format(page_no))
            self.get_balls(html)
            
            nextpage = "[data-page='%d']"%(page_no)
            element = self.driver.find_element(By.CSS_SELECTOR, nextpage)
            if not element:
                print("find element error!")
                return
            element.click()
            print("Sleep to avoid being forbbiden.")
            time.sleep(5)
            html = self.driver.page_source
        
        self.f.close()
        self.driver.quit()
        

if __name__ == '__main__':
    dcb = DoubleColorBall()
    dcb.run()
