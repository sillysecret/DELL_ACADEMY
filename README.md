# DELL_ACADEMY


<img src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white">
<img src="https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white">
<img src="https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324">
<img src="https://img.shields.io/badge/Arch_Linux-1793D1?style=for-the-badge&logo=arch-linux&logoColor=white">

<img src="https://i.pinimg.com/236x/71/01/32/710132864d55b26c57f4a059cf1976f9.jpg">


curl -X POST localhost:3000/aposta \
-H "Content-Type: application/json" \
-d '{"fk_user_id": "018e4067-b94b-73c1-b212-1229d596f4dc", "fk_mega_id": "018e4069-f935-7c01-bb98-8bea3c2a57c8", "vec":[1, 2, 3, 4, 5]}'

curl -X POST localhost:3000/mega \
-H "Content-Type: application/json" \
-d '{"aval": true, "user_id": "018e4068-281e-7ac7-9ceb-076f3658ae40"}'

curl -X POST localhost:3000/user \
-H "Content-Type: application/json" \
-d '{"nome": "John Doe", "cpf": "123.456.789-00"}'
