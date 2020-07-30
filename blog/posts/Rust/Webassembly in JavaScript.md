<!--
  Post{
    subtitle: "Learn how to deploy Rust with Nginx and systemd service.",
    image: "post/deploy/nginx-rust.png",
    image_decription: "Image by Steadylearner",
    tags: "How deploy Rust code",
  }
-->

<!-- Link -->

[Rust]: https://www.rust-lang.org/
[Steadylearner]: https://www.steadylearner.com

[EssentialC]: http://cslibrary.stanford.edu/101/EssentialC.pdf
[The C Programming Lanugage]: https://www.google.com/search?q=the+c+programming+language
[DigitalOcean]: http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM
[Nginx]: https://www.digitalocean.com/community/tutorials/how-to-install-nginx-on-ubuntu-18-04-quickstart
[Rust Web App]: https://github.com/steadylearner/Rust-Web-App
[Rocket]: https://rocket.rs/v0.4/guide/configuration/

<!-- / -->

<!-- Steadylearner Post -->

[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[How to use Rust Yew]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Yew
[How to use Vim]: https://medium.com/@steadylearner/how-to-learn-vim-for-beginners-c81da6f5fde8

<!-- / -->

Rust users mostly end up loving its features such as performance, reliability and productivity. It is not easy to start to learn it, but there are friendly communities to help you.

On top of that, there are decent package management tools like cargo, cargo-edit etc. The programming language itself and packages(crates in Rust) are maturing also.

The language developers and contributers are also giving their time to enhance [Webassembly][How to use Rust Yew] integration to attract more Frontend Developers and make web application faster.

However, it was difficult to find the real examples for intergating frontend app to Rust and then deploying it to the web.

I was not sure that I could upload my website to the web with Rust. But I eventually made it and want to share the process with you.

If you have already experience in deploying websites in other languases, deploying Rust will not be so different from deploying Node js or Python backend app.

1. Buy VPS service such as [DigitalOcean] that offers the same operating system you use in your local machine.

2. Repeat what you have been doing in your local development environment in a vritual machine they give.

3. Configure Apache or Nginx and systemd service worker to work as proxy server

4. Start your project inside the host service and serve it with your domain.

You may read on or just apply the process for a Rust or other web framework you use.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Rust Website][Rust]
3. [DigitalOcean]
4. [SSH](https://www.digitalocean.com/docs/droplets/how-to/add-ssh-keys/create-with-openssh/)
5. [How to use Vim]
6. [Deploy Rocket with Nginx and LetsEncrypt](https://gist.github.com/belst/ff36c5f3883f7bf9b06c379d0a7bed9e)
7. [Bash Command](https://www.howtogeek.com/howto/ubuntu/keyboard-shortcuts-for-bash-command-shell-for-ubuntu-debian-suse-redhat-linux-etc/)
8. [Linux](http://www.usm.uni-muenchen.de/people/puls/lessons/intro_general/Linux/Linux_for_beginners.pdf)

---

I hope you already have Rust installed in your machine and have web application ready.

For this post, I will use [DigitalOcean] as an example. First, I doubted that it would work or not with **Rust**. But later I found that it is easy if you already know how to use Linux Command Line. Because what they give you is just a Linux Virtual Machine and you can repeat what you did in your local machine.

Then, it will work. You are watching the result if you are at [Steadylearner].

It works well with very low price with two month free trial time for you to test. You will not need that much time to begin with if you follow the instruction from this post.

It will be helpful if you are already familiar with **Vim**, **CLI**, **Linux** etc.

You should search for information about [Nginx]. It works well with default configuration but I hope you search information about it on your own.

If you don't have ssh keys in your machine yet, please follow the documentation from [DigitalOcean](https://www.digitalocean.com/docs/droplets/how-to/add-ssh-keys/create-with-openssh/) I gave you.

What you need to do is just type **ssh-keygen** and follow the instruction in your machine.

You can repeat the process if your hardware is broken or when you need to start from nothing when you want to use it in antoher machine.

It will work in those situations also so do not worry and just follow the instructions your service providers give you.

I hope you already know what is tld([top-level-domain](https://searchmicroservices.techtarget.com/definition/top-level-domain-TLD)).

You may read a lot after you made your app deployed with this post.

<br />

<h2 class="blue">Table of Contents</h2>

1. DigitalOcean Setup
2. Install dependencies for your virtual machine
3. Configure Nginx as a reverse proxy server
4. Save your project in your virtual machine
5. Create systemd service to serve your web application
6. HTTPS for your website
7. Conclusion

---

You can skip the **DigtialOcean Setup part** if you do not want to use it or just refer to it.

You will not need the **Installing Dependencies for Linux Virtual Machine** if you already know how to set up development environment.

Just repeat what you have done in your local machine after you buy VPS service from [DigtialOcean] or whatever service host provide.

If you have problem while folllowing this post, please contact me with [Twitter](https://twitter.com/steadylearner_p) or LinkedIn(https://www.linkedin.com/in/steady-learner-3151b7164/) and I will help you.

<br />

## 1. DigitalOcean Setup

<!-- Use other image instead -->
[![DigitalOcean Website Screentshot by steadylearner](https://www.steadylearner.com/static/images/post/deploy/DigitalOcean-Main.png)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

<br />

I wouldn’t write details about using **DigitalOcean** for there are many [documentations](https://www.digitalocean.com/docs/droplets/how-to/create/) for beginners already.

Just use Ubuntu 16.04 or Ubuntu 18.04 or whatever you want and the lowest price version to follow this article.

You can use this as an example and hope you already made one.

<br />

[![DigitalOcean Website Screentshot by steadylearner](https://www.steadylearner.com/static/images/post/deploy/DigitalOcean-Create-Droplet.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

<br />

You can use whatever option you want but saving your resource is important.

Use the lowest price option. It will be enough to test your project.

<br />

[![DigitalOcean Website Screentshot by steadylearner](https://www.steadylearner.com/static/images/post/deploy/DigitlaOcean-Lowest-Price.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

<br />

and we will also use Ubuntu 16.04 for our choice.

<br />

[![DigitalOcean Website Screentshot by steadylearner](https://www.steadylearner.com/static/images/post/deploy/DigitalOcean-Ubuntu-16.04.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

<br />

Your computer should have been connected with your virtual machine that the service provides with SSH like the image below following the instruction.

With command **$ssh yoursite@xxx.xxx.x.xx**, it will show you messages like the image below.

<br />

[![DigitalOcean Website Screentshot by steadylearner](https://www.steadylearner.com/static/images/post/deploy/SSH-Connection-sucess-for-Linux-Ubuntu-16.04.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

<br />

You will see **username@project:~$** in your Linux Kernel.

We are ready with DigitalOcean and we can write code to deploy our Rust Web Application with Nginx.

Your Linux Virtual Machine is almost empty. It will be easy to think that you have to start from nothing.

It has Git to download your project easily, Vim to edit Nginx and Rust files already. We will see how to use them later.

If you forget the domain of your website, you can use

```console
$host www.steadylearner.com
```

Test it with **$host yourwebsite** instead.

<br />

## 2. Install dependencies for your virtual machine

This is very tedious work if you had many dependencies.

For your machine is just empty virtual machine, You have to install **Rust Compiler, Cargo, Node, Nginx** etc to prepare the deployment.

If you are familiar with **Docker**, you could have used it instead to save your time.

What is important is to install Rust to compile your [Rust Web App] and [Nginx] to work as reverse proxy server for it.

To advance, You have to verify minimum requirments are ready with command

**$sudo nginx -h**

[![nginx with nginx -h command](https://www.steadylearner.com/static/images/post/deploy/Nginx.png)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

**$rustup**

[![rustup after installation](https://www.steadylearner.com/static/images/post/deploy/Rustup.png)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

**$cargo**

[![cargo command after installation](https://www.steadylearner.com/static/images/post/deploy/Cargo.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

and you are ready to write the real codes to deploy your website.

In this post, we will use **Rust Rocket framework** but you can use Actix or whatever framework and languages you want instead

They will work because what you learn really in this post is how to use **Nginx** as **proxy server** for your web app in **POSIX** system.

<br />

## 3. Configure Nginx as a reverse proxy server

<section class="flex center">
  <img alt="Nginx from its website" src="https://www.steadylearner.com/static/images/post/deploy/nginx.png">
</section>

You are almost there. You could make it work with the documenations I gave you above.

But following the post will save your time, so I hope you did not tweak anything from the default yet.

You will se that is just the matter of **copy and paste** when you have [a real example][Steadylearner].

Refer to the code snippet below for Nginx save it in /etc/nginx/sites-available/ as your-domain.tld.conf

**your-domain.tld** could be **steadylearner.com**

Just use yours instead.

```nginx
server {
    #listen 80; # Only if sysctl net.ipv6.bindv6only = 1
    listen 80;
    listen [::]:80;

    server_name yourdomain.tld www.yourdomain.tld; # 1.

    location / {
        # Forward requests to rocket v4.0 production port
        proxy_pass http://0.0.0.0:8000; # 2.
        proxy_buffering off; # Single Page App work faster with it
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

The example is very simple but there are two points you should know.

**1.** You need to enable **www** at [DigitalOcean] to prefix it to your sitename.

You may read [how-to-manage-records](https://www.digitalocean.com/docs/networking/dns/how-to/manage-records/), [using-cname-for-www](https://www.digitalocean.com/community/questions/using-cname-for-www-redirection).

You can refer to the image below.

<br />

[![Visit DigitalOcean to prefix www with http or https to redirect your domain](https://www.steadylearner.com/static/images/post/deploy/DigtialOcean-www-Cname.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

<br />

**CNAME** is used for simpilicity, you can use **A** or **AAAA** record for www instead if you want to enable **HTTPS** later in this post.

**2.** The Rust Web framework we will use is [Rocket] and the port for its production is http://0.0.0.0:8000. You had to help nginx to proxy the request to your Rust web application.

You could use other port and configuration if you use another framework such as Actix or ones from other languages etc.

Whenever you edit nginx relevant files, you can test it with
**$sudo nginx -t** and it will show messages similar to this.

```console
nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
nginx: configuration file /etc/nginx/nginx.conf test is successful
```

If it passed, you can use make sym link for the file **your-domain.tld.conf** you made before with

```console
cd /etc/nginx/sites-enabled && sudo ln -s ../sites-available/your-domain.tld.conf
```

I hope you made it well to this point.

You will need various commands of **nginx** after you deploy your website.

Copy and paste them inside your **~/.bashrc** file with **$vim ~/.bashrc** and use $source ~/.bashrc to use them if you want.

**You don’t have to remember details if you know what you want to do.**

```bash
#nginx
alias startn="sudo systemctl start nginx"
alias stopn="sudo systemctl stop nginx"
alias restartn="sudo systemctl restart nginx"
alias reloadn="sudo systemctl reload nginx"
alias statusn="service nginx status"
alias testn="sudo nginx -t"
alias foldern="cd /etc/nginx"
```

There were no diffiuclt points for Nginx. You didn’t even have to edit default **nginx.conf** file.

If you like to serve **gzip** file for your Rust Rocket or other web application.

You may include code snippet below and test them.

```nginx
#inside nginx.conf(/etc/nginx/nginx.conf)

##
# Gzip Settings
##

gzip on;
gzip_disable "msie6";

gzip_vary on;
gzip_proxied any;
gzip_comp_level 5; # it is better not to be larger than 5
gzip_buffers 16 8k;
gzip_http_version 1.1;

# write what you want to be served as gzip compressed file when use requested it.

gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;
```

You can verify it work with

```console
$curl http://yourdomain.com --silent --write-out "%{size_download}\n" --output /dev/null

$curl http://yourdomain.com --silent -H "Accept-Encoding: gzip, other things" --write-out "%{size_download}\n" --output /dev/null
```

and the result will be much better and easier than do it without Nginx.

If you managed to serve gzip files on your own, you can use

```console
$find . | gzip *.ext
$find . | gzip -d *.gz
```

to compress and decompress files in a target folder.

You do not need webpack compressor or else to do that.

<br />

## 4. Save your project in your virtual machine

Your virtual machine was almost empty but you could edit and save the files with **Vim**.

You could also use **git** to download files for they are already pre-installed.

You may donwload your project files from you GitHub repository with command like

```console
$git clone https://github.com/steadylearner/Rust-Web-App.git.
```

You can use whatever you want to save files in your virtual machine instead.

You might want to do it on your home directory. For that, use $cd ~ && mkdir yourwebsite and save your project files there.

You know better what you are doing than others. It is just repeating what you have done to build the entire project. The only difference is paths to serve and link files and directories.

If you are using Rust [Rocket] for this example,

1. Download the files in your virtual machine

2. Use **$cargo check** first to verify everything works fine first.

3. **$cargo run --release** to make production files in target directory inside your virtual machine.

4. Then, we will use [systemd](https://www.digitalocean.com/community/tutorials/understanding-systemd-units-and-unit-files) service to automate the process in the next part

Wanna update files later after you deploy your wbsite?

1. You have to repeat the command **$cargo run --release** whenever you tweaked your Rust files.

2. For static files such as HTML, CSS, JavaScript, Images and other simple files. You just substitute them and they will work.

The difference is that [Rust] is a compiled language and others are not.

<br />

## 5. Create systemd service to serve your web application

You almost made it. We will refer to the [Rocket] for development environment to use it for systemd files.

It explains that there are development, staging and production environment and will be similar in other web frameworks.

For we are dealing with prodcution files and ready to deploy it to the web, only care for production part.

When you use [Rocket], it already has its own default configuration and you would not need to edit that much.

Read [its documenation] and find those parts in description will be used in **systemd service** later.

If you want to learn more about what **servie** mean here, you may refer to

"A service unit describes how to manage a service or application on the server. This will include how to start or stop the service, under which circumstances it should be automatically started, and the dependency and ordering information for related software"

from [here](https://www.digitalocean.com/community/tutorials/understanding-systemd-units-and-unit-files).

Then, make **/etc/systemd/system/your-domain.tld.service** and write content in it similar to

```bash
[Unit]
Description=Web Application Example from steadylearner

[Service]
User=www-data
Group=www-data
WorkingDirectory=/home/yourname/yourwebsite/yourproject/
Environment="ROCKET_ENV=prod"
Environment="ROCKET_ADDRESS=0.0.0.0"
Environment="ROCKET_PORT=8000"
Environment="ROCKET_LOG=critical"
ExecStart=/home/yourname/yourwebsite/yourproject/target/release/yourproject

[Install]
WantedBy=multi-user.target
```

You should find the right path for your project and everything is ready.

Test in your virtual machien console with

```console
$sudo systemctl start your-domain.tld.service.
```

That will make your [Rocket] or other framework you use to compile production files and serve them to port instead of you.

You may visit your domain your-domain.tld at this point.

If you want, you can save the command similar to

```bash
alias verifywebsite="curl curl https://www.steadylearner.com"
```

in your ~/.bashrc file or you can use **$ping** instead if you are familiar with Linux Commands.

The result will be response from Nginx or your production web pages if you could make it work.

You can verify the real exmaple at [Steadylearner].

**Hope you made it.**

You may write more alias in your **~/.bashrc** similar to

```bash
# ufw firewall(install ufw first)
alias allownginx="sudo ufw allow 'Nginx Full'"
alias reloadufw="sudo ufw reload"

# systemd service
alias start="sudo systemctl start yourdomain.tld.service"
alias stop="sudo systemctl stop yourdomain.tld.service"
alias status="sudo systemctl status yourdomain.tld.service"
alias reload="sudo systemctl daemon-reload"
# It works automatically for every reboot, use it just once.
alias autoreload="sudo systemctl enable yourdomain.tld.service"
```

That was all to deploy your Rust Web application to the web with **Nginx** and **systemd** service in Linux Virtual Machine.

You may use other web frameworks also. You just need to edit the paths and configuration parts for it.

If you want to allow HTTPS for your website, please follow the next part.

<br />

## 6. HTTPS for your website

What we need to enable https for our website are just a few lines of commands.

```console
$sudo certbot --nginx # 1.
$sudo certbot renew --dry-run # 2.
```

1. Refer to the official documenation from [cerbot](https://certbot.eff.org/docs/using.html#nginx).

2. Read [the documentation](https://www.digitalocean.com/community/tutorials/how-to-secure-nginx-with-let-s-encrypt-on-ubuntu-16-04) from DigitalOcean about **renew --dry -run** command

From the first process with **$sudo certbot --nginx**, you will see the processes similar to

[![https-cert](https://www.steadylearner.com/static/images/post/deploy/https-cert-process.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

[![https-enable](https://www.steadylearner.com/static/images/post/deploy/https-enable-process.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

It would have shown certificate will expire later and you have to do some process for that.

We are ready for it. You just need to use **$sudo certbot renew --dry-run**.

[![https-renew](https://www.steadylearner.com/static/images/post/deploy/https-renew.png
)](http://pages.news.digitalocean.com/dcn/AyKQ30vur1Nt8H30LIWxk-j5xHmafGnoECQwn1ooO76IYFHzigM_y4fqCVuHjuXsYYmKVEtVdAWxss0KUtUjfw==/DE3v00e0DIX002276X3M0VM)

You can see that it is just simulating the renewal process in the description.

Then, you can visit your website and [https is enabled][Steadylearner] for it.

<br />

## 7. Conclusion

I know that it is not easy to follow this post.

You did well if you made it. Otherwise you can eventually achieve what you want.

You can deploy a web application made with **Rust**.

I felt that there is nothing special in using Rust for web development with Nginx and Linux.

Only ports to serve files would be different from others and the rest of the process will be similiar.

You may do the same for other programming languages and frameworks also.

What you learnt is how to use Nginx and systemld for whatever web frameworks written in any programming languages.

It was the sum of the previous posts I wrote before.

You may visit [one of them](https://medium.com/@steadylearner/how-to-deploy-rust-web-application-8c0e81394bd5) if you have problem with this post or you may search [How to deploy Rust](https://www.google.com/search?&q=how+to+deploy+rust).

**Thanks and please share this post with others**.