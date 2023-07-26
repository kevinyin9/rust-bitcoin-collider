import datetime
import json
import os
import hashlib
from threading import Thread, active_count
import time
import traceback

import requests

def sha256(data):
    digest = hashlib.new("sha256")
    digest.update(data)
    return digest.digest()


def ripemd160(x):
    d = hashlib.new("ripemd160")
    d.update(x)
    return d.digest()


def b58(data):
    B58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"

    if data[0] == 0:
        return "1" + b58(data[1:])

    x = sum([v * (256 ** i) for i, v in enumerate(data[::-1])])
    ret = ""
    while x > 0:
        ret = B58[x % 58] + ret
        x = x // 58

    return ret


class Point:
    def __init__(self,
        x=0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798,
        y=0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8,
        p=2**256 - 2**32 - 2**9 - 2**8 - 2**7 - 2**6 - 2**4 - 1):
        self.x = x
        self.y = y
        self.p = p

    def __add__(self, other):
        return self.__radd__(other)

    def __mul__(self, other):
        return self.__rmul__(other)

    def __rmul__(self, other):
        n = self
        q = None

        for i in range(256):
            if other & (1 << i):
                q = q + n
            n = n + n

        return q

    def __radd__(self, other):
        if other is None:
            return self
        x1 = other.x
        y1 = other.y
        x2 = self.x
        y2 = self.y
        p = self.p

        if self == other:
            l = pow(2 * y2 % p, p-2, p) * (3 * x2 * x2) % p
        else:
            l = pow(x1 - x2, p-2, p) * (y1 - y2) % p

        newX = (l ** 2 - x2 - x1) % p
        newY = (l * x2 - l * newX - y2) % p

        return Point(newX, newY)

    def toBytes(self):
        x = self.x.to_bytes(32, "big")
        y = self.y.to_bytes(32, "big")
        return b"\x04" + x + y


def getPublicKey(privkey):
    SPEC256k1 = Point()
    pk = int.from_bytes(privkey, "big")
    hash160 = ripemd160(sha256((SPEC256k1 * pk).toBytes()))
    address = b"\x00" + hash160

    address = b58(address + sha256(sha256(address))[:4])
    return address


def getWif(privkey):
    wif = b"\x80" + privkey
    wif = b58(wif + sha256(sha256(wif))[:4])
    return wif

def post(thread_name):
    print("In thread: ", thread_name)
    if __name__ == "__main__":
        count = 0
        while(1):
            url = 'https://blockchain.info/multiaddr?active='
            privatekey = {}
            privatekey.clear()
            for i in range(100):
                if count % 10000 == 0 and i == 0: # check the program can detect money of address and show it
                    privatekey['3F4B11x794E4wUD5rYpqtBnCaUZX4wSppv'] = 'e0444444444444444444444444444444444444444444'
                    url += '3F4B11x794E4wUD5rYpqtBnCaUZX4wSppv|'
                else:
                    randomBytes = os.urandom(32)
                    address = getPublicKey(randomBytes)
                    privkey = getWif(randomBytes)
                    privatekey[address] = privkey
                    count+=1
                    url += address + "|"
            count += 100
            url = url[:-1]
            headers = {'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36'} 
            proxies =  { 'http' :  'http://103.44.23.7:8888' }
            time.sleep(1)
            try:
                r = requests.get(url, proxies = proxies, headers=headers)
                y = json.loads(r.text)
            except Exception as e: 
                print(count)
                print(r.text)
                traceback.print_exc() #print error
            else:
                if count % 10000 == 0:
                    print(thread_name, count, datetime.now())
                for i in range(100):
                    if y['addresses'][i]['final_balance'] != 0 or y['addresses'][i]['n_tx'] != 0:
                        file.write(privatekey[y['addresses'][i]['address']] + "  " + str(y['addresses'][i]['final_balance'])  + ", n_tx= " + str(y['addresses'][i]['n_tx']) + "\n")
                        print(privatekey[y['addresses'][i]['address']] + " " + str(y['addresses'][i]['final_balance']) + ", n_tx= " + str(y['addresses'][i]['n_tx']))
                        file.flush() #refresh so that the file.write will appear immediately
            
            
file = open("record_btc_1.txt", 'a', encoding = 'UTF-8')
threads = []

t = Thread(target = post, args = ('Thread 1', ))
t.daemon = True
t.start()
threads.append(t)

# t = Thread(target = post, args = ('Thread 2', ))
# t.daemon = True
# t.start()
# threads.append(t)

# t = Thread(target = post, args = ('Thread 3', ))
# t.daemon = True
# t.start()
# threads.append(t)

# t = Thread(target = post, args = ('Thread 4', ))
# t.setDaemon(True)
# t.start()
# threads.append(t)

# t = Thread(target = post, args = ('Thread 5', ))
# t.setDaemon(True)
# t.start() 
# threads.append(t)

# for t in threads:
#     t.join()
while active_count() > 0:
    time.sleep(0.01)