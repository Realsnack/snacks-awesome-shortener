db = db.getSiblingDB('admin')

db.createUser(
  {
    user: "monitoring",
    pwd: "monitoring",
    roles: [
      {
        "role": "clusterMonitor",
        "db": "admin"
      },
      {
        "role": "read",
        "db": "local"
      },
      {
        "role": "read",
        "db": "shorts"
      }
    ]
  }
)

db = db.getSiblingDB('shorts');

db.createUser({
    user: "shorts_app",
    pwd: "shorts_pwd",
    roles: [
        { role: "readWrite", db: "shorts" }
    ]
});

db.createUser({
    user: "compass_user",
    pwd: "compass_pwd",
    roles: [
        { role: "read", db: "shorts" }
    ]
});

