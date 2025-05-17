#!/bin/bash

APP_ID="SET-YOUR-ID"
ENDPOINT="https://shopping.yahooapis.jp/ShoppingWebService/V3/itemSearch"
URL="${ENDPOINT}?appid=${APP_ID}&query=nike&genre_category_id=1"
curl "${URL}" | jq

