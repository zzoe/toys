# 默认端口 8000 --bind 0.0.0.0:8008
#surreal start -A --auth --log trace --user root --pass root_pass file://../rocks.db
surreal start -A --auth --log trace file://$PSScriptRoot/../rocks.db
