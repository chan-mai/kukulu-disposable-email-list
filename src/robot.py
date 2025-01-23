import requests
from bs4 import BeautifulSoup
import os
import random

if __name__ == '__main__':
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
    print(headers)
    response = requests.get('https://m.kuku.lu/ja.php', headers=headers)
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
    
    print(f"diff: {len(new_domains)}\n{new_domains}")
    
    # 重複を削除
    new_domains = list(set(new_domains))
    new_domains.sort()
    
    # 保存
    with open('domains.txt', 'a') as f:
        for domain in new_domains:
            f.write(domain + '\n')
