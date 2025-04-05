//describe this direcotry

//this is a test for compatibility across different operating systems and versions.

Run the Server explicity using the command:

```rs
docker run -it -p 7878:7878 ghcr.io/micheal-ndoh/chatme:slim server
```

or Just run the Server:

```rs
docker run -it -p 7878:7878 ghcr.io/micheal-ndoh/chatme:slim
```

Run the Client:

```rs
docker run -it -p 7878:7878 ghcr.io/micheal-ndoh/chatme:slim client
```


Test on different containers on thesame network:

**Firstly create a network:**
```rs
docker network create chatme-network
```

**Run the Server:**
```rs
docker run -d --network chatme-network ghcr.io/micheal-ndoh/chatme:slim server
```

**Run the Client:**
```rs
docker run -d --network chatme-network ghcr.io/micheal-ndoh/chatme:slim client
```