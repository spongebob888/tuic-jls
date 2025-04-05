import subprocess as sp
from time import sleep

sp.run(["cargo","build","-p","tuic-server"])
h2 = sp.Popen(["cargo","run","-p","tuic-server","--", "-c","server_local.json"],
                     start_new_session=True )
sleep(0.5)
out = sp.run(["curl","https://echo.free.beeceptor.com:4443",
               "--http3-only",
               "--resolve",
               "echo.free.beeceptor.com:4443:127.0.0.1"
               ],
             capture_output=True,
             text=True)


h2.terminate()
print(out.stderr)

print(out.stdout)
assert("GET" in out.stdout)