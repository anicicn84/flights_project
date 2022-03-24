curl --location --request POST 'localhost:3030/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "stop_list": [["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]
}'