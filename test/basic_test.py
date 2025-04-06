import subprocess as sp
from time import sleep
sp.run(["cargo","build","-p","tuic-server"])
sp.run(["cargo","build","-p","tuic-client"])
h1 = sp.Popen(["cargo","run","-p","tuic-client","--", "-c","client_local.json"],
                     start_new_session=True )
h2 = sp.Popen(["cargo","run","-p","tuic-server","--", "-c","server_local.json"],
                     start_new_session=True )
sleep(0.5)
out = sp.run(["curl http://echo.free.beeceptor.com --socks5 127.0.0.1:1089"],shell=True,
             capture_output=True,
             text=True)


h1.terminate()
h2.terminate()
print(out.stderr)

print(out.stdout)
assert("GET" in out.stdout)