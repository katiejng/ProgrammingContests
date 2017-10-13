import requests


def get_topic(title, description):
    url = "http://34.248.92.90:6000/classify_json"

    querystring = {"api_key": "TRTFGDERT%$"}

    payload = "[{\"title\":\"" + title + "\",\"description\":\"" + description + "\"}]"

    headers = {
        'content-type': "application/json",
        'cache-control': "no-cache",
    }

    response = requests.request("PUT", url, data=payload, headers=headers, params=querystring)

    return (response.text)


print(get_topic("Santa", "murder"))