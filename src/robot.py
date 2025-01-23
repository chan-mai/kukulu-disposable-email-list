import requests
import urllib3
from urllib3.exceptions import InsecureRequestWarning
urllib3.disable_warnings(InsecureRequestWarning)
from bs4 import BeautifulSoup
import os
import random
import datetime

start = datetime.datetime.now()

def log(message):
    print(f'\033[32m[log]\033[0m {(datetime.datetime.now() - start).seconds}s', message)

def crawl(proxy):
    user_agents = [
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3864.0 Safari/537.36',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.61 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:62.0) Gecko/20100101 Firefox/62.0',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:67.0) Gecko/20100101 Firefox/67.0',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:68.0) Gecko/20100101 Firefox/68.0',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:61.0) Gecko/20100101 Firefox/61.0',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:62.0) Gecko/20100101 Firefox/62.0',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:92.0) Gecko/20100101 Firefox/92.0',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/17.17134',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.61 Safari/537.36 Edg/94.0.992.31',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Safari/605.1.15'
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.100 Safari/537.36',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36'
    ]
        
    # ランダムなUAでhttps://m.kuku.lu/ja.phpを取得
    headers = {
        'User-Agent': user_agents[random.randint(0, len(user_agents) - 1)]
    }
    # print(headers)
    # proxyがあれば設定
    response = None
    
    if proxy:
        response = requests.get('https://m.kuku.lu/ja.php', headers=headers, proxies=proxy, verify=False, allow_redirects=False)
    else:
        response = requests.get('https://m.kuku.lu/ja.php', headers=headers)
    
    if response is None:
        print("Error: response is None")
        return
    
    if response.status_code != 200:
        print(f"Error: {response.status_code}")
        return
    
    response.encoding = response.apparent_encoding
    html = response.text
    
    # Array.from(document.querySelectorAll("#input_manualmaildomain_list b[dir='auto']")).map(el => el.innerText.slice(1)).join("\n")
    soup = BeautifulSoup(html, 'html.parser')
    elements = soup.select("#input_manualmaildomain_list b[dir='auto']")
    domains = [el.text[1:] for el in elements]
    
    # 読み込み
    current_domains = []
    if os.path.exists('domains.txt'):
        with open('domains.txt', 'r') as f:
            current_domains = f.read().split('\n')
            
    # 差分を取得
    new_domains = list(set(domains) - set(current_domains))
    
    log(f"get: {len(domains)}, current: {len(current_domains)}, diff: {len(new_domains)}, add: {len(new_domains)}")
    log(new_domains)
    
    # 重複を削除
    new_domains = list(set(new_domains))
    new_domains.sort()
    
    # 保存
    with open('domains.txt', 'a') as f:
        for domain in new_domains:
            f.write(domain + '\n')

def get_proxy_ip():
    # https://api.proxyscrape.com/v4/free-proxy-list/get?request=display_proxies&country=jp&protocol=http&proxy_format=protocolipport&format=json&timeout=20000
    # API叩く
    response = requests.get('https://api.proxyscrape.com/v4/free-proxy-list/get?request=display_proxies&country=jp&protocol=http&proxy_format=protocolipport&format=json&timeout=20000')
    response.encoding = response.apparent_encoding
    
    # response.json()["proxies"]からランダム取得
    proxies = response.json()["proxies"]
    proxy = {
        "http": proxies[random.randint(0, len(proxies) - 1)]["proxy"].replace("http://", ""),
    }
    return proxy


if __name__ == '__main__':
    # プロキシ / 非プロキシで各1回クロール
    log("Run without proxy")
    crawl(None)
    log("Done without proxy")
    
    log("Run with proxy")
    proxy = get_proxy_ip()
    log(f"Proxy: {proxy['http']}")
    crawl(proxy)
    log("Done with proxy")
    
