#!/bin/bash

# Зчитуємо TELOXIDE_TOKEN з .env файлу
TELOXIDE_TOKEN=$(grep TELOXIDE_TOKEN .env | cut -d '=' -f2)

# Запускаємо cargo run з токеном
TELOXIDE_TOKEN=$TELOXIDE_TOKEN cargo run