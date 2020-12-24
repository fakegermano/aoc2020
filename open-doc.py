#!/usr/bin/env python3

import os

import http.server
import socketserver

PATH = "/home/wsl/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/"

os.chdir(PATH)

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print("serving at port", PORT)
    httpd.serve_forever()