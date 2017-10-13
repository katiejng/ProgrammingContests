import http.client
import json
from urllib.parse import urlparse
import urllib.parse
import urllib.request
import csv
import re
from itertools import combinations


def get_azure_keywords(a_list):
    pass

def get_sources():
    # Download all sources
    conn = http.client.HTTPSConnection("newsapi.org")
    conn.request("GET", "/v1/sources?language=en")
    r1 = conn.getresponse()
    # print(r1.status, r1.reason)

    # put all sources into json_sources
    data = r1.read().decode('utf-8')  # This will return entire content.
    sources_json = json.loads(data)
    # print(sources_json["sources"][0])

    # print(type(sources_json["sources"]))
    return sources_json


def get_articles(source_id):
    conn = http.client.HTTPSConnection("newsapi.org")
    conn.request("GET", "/v1/articles?source=" + source_id + "&apiKey=8e5889be46554bf9aabcdee0347b18ff")
    r1 = conn.getresponse()
    # print(r1.status, r1.reason)

    # put all sources into json_sources
    data = r1.read().decode('utf-8')  # This will return entire content.
    article_json = json.loads(data)
    if article_json["status"] == "ok":
        return (article_json["articles"])
    else:
        return -1


def get_article_title(article):
    return article["title"]


def get_article_desc(article):
    return article["description"]


def get_keywords(article):
    # get the URL
    url = article["url"]

    # find the last 3 parts of the URL
    parsed_url = urlparse(url)

    # print(re.split("/|-", parsed_url.path))

    spliturl = re.split("/|-", parsed_url.path)

    spliturl[-1] = spliturl[-1].split(".")[0]

    new_keywords = []
    for word in spliturl:
        if re.match("^[A-Za-z]{3,}$", word) != None:
            if word!="news" and word!="article":
                new_keywords.append(word)


    return new_keywords

if __name__ == "__main__":
    all_my_data=[]

    with open('azure.csv', 'w', newline='') as f:

        # Get all articles from NY Times

        sources_json = get_sources()
        # iterate over each source

        for source_json in sources_json["sources"][:1]:
            source_id = source_json["id"]
            print(source_id)

            try:
                article_list = get_articles(source_id)

                # Collect titles, descriptions and keywords
                for article in article_list:
                    title = get_article_title(article)
                    desc = get_article_desc(article)

                    all_my_data.append([title,desc])
                    print(len(all_my_data))
            except Exception:
                print('exception')
                pass


        all_my_data_keywords = get_azure_keywords(all_my_data)
        with open("test.txt","w") as file:
            file.write(str(all_my_data_keywords))