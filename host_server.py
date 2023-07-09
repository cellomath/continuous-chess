import http.server

port = 8000
address = ("localhost",port)
server = http.server.HTTPServer
handler = http.server.SimpleHTTPRequestHandler
handler.extensions_map=({
    '.manifest': 'text/cache-manifest',
    '.html': 'text/html',
    '.png': 'image/png',
    '.jpg': 'image/jpg',
    '.svg': 'image/svg+xml',
    '.css': 'text/css',
    '.js': 'text/javascript',
    '': 'application/octet-stream', # Default
    })
httpd = server(address, handler)
httpd.serve_forever()