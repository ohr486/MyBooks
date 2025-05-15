#!/bin/bash

APP_ID="TODO-SET-KEY"

curl -k "https://shopping.yahooapis.jp/ShoppingWebService/V1/json/categorySearch?appid=${APP_ID}&category_id=1"
