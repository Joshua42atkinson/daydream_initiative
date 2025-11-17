import http.server
import socketserver
import os

PORT = 8000
web_dir = os.path.join(os.path.dirname(__file__), 'dist')
os.chdir(web_dir)
LOG_FILE = "server_requests.log"

with open(LOG_FILE, "w") as f:
    f.write("Server starting...\\n")

class LoggingRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        with open(LOG_FILE, "a") as f:
            f.write(f"GET request for: {self.path}\\n")
        super().do_GET()

Handler = LoggingRequestHandler
httpd = socketserver.TCPServer(("", PORT), Handler)
with open(LOG_FILE, "a") as f:
    f.write(f"serving at port {PORT}\\n")
httpd.serve_forever()
