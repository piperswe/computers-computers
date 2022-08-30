+++
title = 'Installing Multicraft on Debian 9 ("Stretch")'
date = "2018-07-26"
author = "Piper McCorkle"
cover = ""
tags = ["linux", "minecraft", "debian"]
keywords = []
description = "A up-to-date guide for installing and running Multicraft on Debian 9, a.k.a. Stretch"
showFullContent = false
readingTime = true
hideComments = false
draft = false
+++

> This post is extremely out of date. Debian 9 has been succeeded by Debian 10,
> and Debian 10 has been succeeded by Debian 11. There are probably better
> guides out there, and I can't guarantee this one will still work.
>
> If you're interested in an updated guide, get in touch using the details on
> this blog's about page.

## Introduction

I'm a pretty big advocate of Debian, but when I suggested to a friend that he
should host his Minecraft server on Debian, I soon saw that there are no
up-to-date guides to installing Multicraft, a popular Minecraft server panel,
on Debian. The most popular seems to be on the Linode documentation, but it was
even before Debian switched to systemd.

So, here's an up-to-date guide for installing and running Multicraft on Debian
9, a.k.a. Stretch.

## Before You Begin

Before starting this guide, you will need to have the following:

- Debian 9, installed and fully updated

  For reference, the following commands will get your Debian installation
  up-to-date:

  ```bash
  sudo apt update
  sudo apt dist-upgrade
  ```

- A non-root user with `sudo` permission
- An SSH server set up

It is assumed you are running any commands mentioned from a non-root user over
an SSH connection. If you run them as root, there's no telling what the
outcome will be.

## Installing Prerequisites

Multicraft requires a webserver, PHP, SQLite (or MySQL, which will not be
covered here), and Java. Thankfully, these are all available in the Debian
repositories, so we can download them with a single command.

```bash
sudo apt install nginx sqlite php7.0-fpm php7.0-sqlite openjdk-8-jre-headless zip
```

This command installs `nginx` (a high-performance web server), `sqlite` (the
database software used by Multicraft), `php7.0-fpm` (the programming language
environment used by Multicraft, in a form suitable for Nginx), `php7.0-sqlite`
(an adapter to connect PHP with SQLite), `openjdk-8-jre-headless` (an
open-source version of Java 8, which is required to run Minecraft servers), and
`zip` (a utility for extracting zip files).

You might also need to install the `wget` and/or `nano` package(s) if they
aren't installed already.

## Configuring the Web Server

Currently we have _installed_ Nginx and PHP, but they aren't talking. To make
PHP work, we need to edit the Nginx configuration and enable it. To edit the
configuration file, open it with the following command (or using your favorite
editor; Neovim is my personal favorite):

```bash
sudo nano /etc/nginx/sites-enabled/default
```

In the first "server" block, change the "index" line to look like the following:

```nginx
index index.php index.html index.htm index.nginx-debian.html;
```

Then, delete a single octothorpe ("`#`") from each of the following lines, except
the second to last:

```nginx
location ~ \.php$ {
       include snippets/fastcgi-php.conf;

       # With php-fpm (or other unix sockets):
       fastcgi_pass unix:/var/run/php/php7.0-fpm.sock;
       # With php-cgi (or other tcp sockets):
       #       fastcgi_pass 127.0.0.1:9000;
}
```

Last, after the `location /` block, add the following block to prevent the
protected directory from being accessed:

```nginx
location /multicraft/protected {
        deny all;
}
```

Exit your editor (Ctrl+X, then Y, then enter with nano), then run the following
to reload the Nginx config file:

```bash
sudo systemctl reload nginx
```

Now that PHP is set up, it's time to install Multicraft.

## Installing Multicraft

> This bit is mostly from the
> [Multicraft documentation](https://web.archive.org/web/20190429012206/https://www.multicraft.org/site/docs/install#2.1),
> with some minor changes.

First off, you will have to download Multicraft. It's easily wgettable:

```bash
wget http://www.multicraft.org/download/linux64 -O multicraft.tar.gz
```

Then you need to extract it and change into the multicraft directory:

```bash
tar -xvf multicraft.tar.gz
cd multicraft
```

There's a nice shell script that will automatically install Multicraft for you,
so let's run it.

```bash
sudo ./setup.sh
```

I'm going to give you a few recommended answers to the setup:

1.  "Run each Minecraft server under its own user?": `Y`
2.  "Run Multicraft under this user:": `minecraft`
3.  "User not found. Create user on start of installation?": `Y`
4.  "Install Multicraft in:": Press enter for the default
5.  "Will the web panel run on this machine?": `Y`
6.  "User of the webserver:": `www-data`
7.  "Location of the web panel files:": `/var/www/html/multicraft`
8.  "Enable builtin FTP server?": `N`
9.  "What kind of database do you want to use?": `sqlite`

Once that setup is done, you should be able to navigate to
`http://[server ip here]/multicraft` and set up the frontend.

## Starting the Daemon

The typical way to start Multicraft is to use a command whenever you turn on
your server. Debian has a program called `systemd` which, when configured
properly, lets you automatically start Multicraft when the system boots. I've
already written a configuration file for you, so you can download it straight
to the configuration folder:

> Actually, sorry. This file has been lost to time.

```bash
wget https://zebmccorkle.u.asymptote.club/util/multicraft.service -O- | sudo tee /etc/systemd/system/multicraft.service
```

Once you've done that, run these commands to load the new configuration file in, start Multicraft, and configure `systemd` to start it on boot:

```bash
sudo systemctl daemon-reload
sudo systemctl start multicraft
sudo systemctl enable multicraft
```

Now go back to your web browser and fully complete your front-end setup.

## Finishing Touches

Finally, to complete your installation, delete your `install.php` file.

> WARNING: Make sure you have completed the front-end setup first!

```bash
sudo rm /var/www/html/multicraft/install.php
```

Congratulations! You now have Multicraft installed on Debian 9.
