# inarust

Inarust Boilerplate, setting environment dahulu

```env
MONGOSTRINGAWANGGA=mongodb://localhost:27017
SERVERADDRESS=0.0.0.0:3000
```

buat db mydatabase dan collection mycollection kemudian isi dengan json :

```json
{
  "name": "John Doe",
  "alamat": "bandung"
}
```

test dengan curl

```sh
curl http://localhost:3000/
curl http://localhost:3000/users
curl -X POST http://localhost:3000/create-user
curl "http://localhost:3000/item/42?number=2"
curl -X POST http://localhost:3000/add-item \
    -H "Content-Type: application/json" \
    -d '{"title": "Some random item"}'
curl -X POST http://localhost:3000/add-user -H "Content-Type: application/json" -d "{\"username\": \"awangga\",\"email\":\"awangga@gmail.com\"}"
curl -X DELETE http://localhost:3000/delete-user/1
curl -X DELETE http://localhost:3000/delete-user/2
```
