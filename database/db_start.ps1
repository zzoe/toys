# 默认端口 8000 --bind 0.0.0.0:8008
#surreal start -A --log trace --user root --pass root_pass surrealkv://../toys.db
#surreal start -A --log trace surrealkv://$PSScriptRoot/../toys.db
surreal start -A surrealkv://toys.db
