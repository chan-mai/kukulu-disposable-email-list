import os
import datetime

start = datetime.datetime.now()

def log(message):
    print(f'\033[32m[log]\033[0m {(datetime.datetime.now() - start).seconds}s', message)


if __name__ == '__main__':
    try:
        current_domains = []
        # domains.txtがあれば読み込む
        if os.path.exists('domains.txt'):
            with open('domains.txt', 'r') as f:
                current_domains = f.read().splitlines()
            log(f"domains.txt found: {len(current_domains)}")
        
        else:
            log("domains.txt not found")
            
        # 昇順ソート
        current_domains.sort()
        # 重複削除
        current_domains = list(set(current_domains))
        
        # 保存
        with open('domains.txt', 'w') as f:
            for domain in current_domains:
                f.write(domain + '\n')
    
    except Exception as e:
        log(f"Error: {e}")
    
    else:
        log(f"domains.txt saved: {len(current_domains)}")
    
