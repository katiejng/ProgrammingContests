import httplib
import json
import requests
import csv

def get_sources():
    # Download all sources
    conn = httplib.HTTPSConnection("newsapi.org")
    conn.request("GET", "/v1/sources?language=en")
    r1 = conn.getresponse()

    # put all sources into json_sources
    data = r1.read().decode('utf-8')  # This will return entire content.
    sources_json = json.loads(data)

    # print(type(sources_json["sources"]))
    return sources_json


def get_articles(source_id):
    conn = httplib.HTTPSConnection("newsapi.org")
    conn.request("GET", "/v1/articles?source=" + source_id + "&apiKey=8e5889be46554bf9aabcdee0347b18ff")
    r1 = conn.getresponse()
    # print(r1.status, r1.reason)

    # put all articles into json_sources
    data = r1.read().decode('utf-8')  # This will return entire content.
    article_json = json.loads(data)
    if article_json["status"] == "ok":
        return (article_json["articles"])
    else:
        return -1

file = open('myres.csv', 'wb')
filewriter= csv.writer(file, delimiter=',',
                            quotechar='"', quoting=csv.QUOTE_MINIMAL)
sources_json = get_sources()


# iterate over each source
for source_json in sources_json["sources"]:
    source_id = source_json["id"]

    print(source_id)

    article_list = get_articles(source_id)

    if article_list == -1:
        continue

    # create my_data
    my_data = {"documents": []}

    my_data2 = {"documents": []}
    my_id = 1
    # Collect titles, descriptions and keywords
    for article in article_list:

        try:


            # add to my_data
            my_data["documents"].append(
                {"language": "en", "id": "{}".format(my_id), "text": "{}. {}".format(article["title"], article["description"])})
            my_data2["documents"].append(
                {"language": "en", "id": "{}".format(my_id),
                 "title": "{}".format(article["title"]), "description": "{}".format(article["description"])})

            my_id+=1
        except ValueError:
            print "Value Error"
        except:
            print "Unknown Error"

    # actually do the request
    if (my_data["documents"]):
        my_data = json.dumps(my_data)
        # print("the data is here: " + str(my_data))
        # print(type(my_data))
        response = requests.post('https://westus.api.cognitive.microsoft.com/text/analytics/v2.0/keyPhrases', \
                                 headers={"Ocp-Apim-Subscription-Key": "2e2a264ffe374b67b988104804ce8856",
                                          "Content-Type": "application/json", \
                                          "Accept": "application/json",
                                          "Content-length": "{}".format(len(str(my_data)))}, data=my_data)
        # print(response.text)
        # print(type(response.text))
        response = response.json()  # check if json fails TODO
        my_data = json.loads(my_data)
        for i in range(len(response["documents"])):
            article = response["documents"][i]
            # print(article)
            # print(type(article))
            ref = article["id"]
            keyPhrases = article["keyPhrases"]
            #print(my_data["documents"])
            filewriter.writerow([my_data2["documents"][i]["title"],my_data2["documents"][i]["description"]]  + keyPhrases )