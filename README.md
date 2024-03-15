# DELL_ACADEMY

curl -X POST localhost:3000/aposta \
-H "Content-Type: application/json" \
-d '{"fk_user_id": "018e4067-b94b-73c1-b212-1229d596f4dc", "fk_mega_id": "018e4069-f935-7c01-bb98-8bea3c2a57c8", "vec":[1, 2, 3, 4, 5]}'

curl -X POST localhost:3000/mega \
-H "Content-Type: application/json" \
-d '{"aval": true, "user_id": "018e4068-281e-7ac7-9ceb-076f3658ae40"}'

curl -X POST localhost:3000/user \
-H "Content-Type: application/json" \
-d '{"nome": "John Doe", "cpf": "123.456.789-00"}'
