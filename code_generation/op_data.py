import requests
from bs4 import BeautifulSoup as bs
import json

filename = 'op_data.json'


class OpData:
    def __init__(self, data):
        self.name = data['name']
        self.hex = data['hex']
        self.addr = data['addr']
        self.f_name = data['name'].lower() + '_' + data['hex'].lower()


def scrape_table():
    url = 'https://wiki.superfamicom.org/65816-reference#instructions-60'
    page = requests.get(url)
    soup = bs(page.content, 'html.parser')
    tables = soup.find_all('table')
    return tables[3]

def parse_table(table):
    rows = table.find_all('tr')
    ops = []

    for r in rows:
        tds = r.find_all('td')
        if not tds:
            continue
        # op = str(tds[0])[4:7]
        op = tds[0].get_text()[:3]
        hx = tds[3].get_text()
        addr = tds[4].get_text()
        ops.append({'name':op, 'hex':hx, 'addr':addr})
    return ops


def save_data(op_data):
    with open(filename, 'w') as f:
        f.write(json.dumps(op_data))

def read_data():
    with open(filename, 'r') as f:
        data = json.load(f)
    result = [OpData(d) for d in data]
    return result