#!/bin/bash

APP_ID="TODO-SET-KEY"

ENDPOINT="https://shopping.yahooapis.jp/ShoppingWebService/V3/itemSearch"
URL="${ENDPOINT}?appid=${APP_ID}&category_id=1"

curl -k "${URL}"
