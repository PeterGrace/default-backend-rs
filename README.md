# default-backend-rs

## Why
I wanted an implementation of a default backend for the nginx ingress-controller that surfaced the headers from the request into the document space, so that I could customize the error page based on that info.  The result is this project, which enables you to create error html templates with Handlebars.

## Where to get it
The latest image is always available on the docker hub, so `docker pull petergrace/default-backend-rs:latest` will get the latest version.

## How to deploy
This backend can be deployed as a default-backend for ingress-nginx globally, or you can also deploy it multiple times
for the purpose of having custom error pages per-ingress.  The container listens on port 8000 for traffic.
See the `deploy/` subfolder for one example of deploying the pod and its service.  You will likely need to customize 
this for your use.

## How to leverage this tool
Once the default-backend is deployed, you can use it by annotating your ingress records as so:
```
metadata:
  annotations:
    nginx.ingress.kubernetes.io/default-backend: my_deployed_dbrs
    nginx.ingress.kubernetes.io/custom-http-errors: "404,418,500"
```
In the above example annotations for the ingress, this would instruct your ingress object to change its default backend
to be the my_deployed_dbrs service **in the same namespace as the ingress.**  The custom-http-errors would enable the
nginx-ingress improved error passthrough information as specified [in the nginx-ingress custom errors handler docs.](http://kubernetes.github.io/ingress-nginx/user-guide/custom-errors/)

## How to customize the error webpages
The container, when it starts, checks for the existence of certain files under /app/templates and /app/public.  If those
files do not exist, it will copy the default files into those paths.

If you create volume mounts at those two paths, you can customize the error pages to your liking.  You could also build 
a Docker image `FROM` this container to customize the default files for your usage; the files live in templates/ and 
public/ in this repository. 

The templates use the handlebars templating language, and the headers are passed into the template as the variable context.

## Special routes
There are some special routes available:
### /health
returns a 200 with json status 'ok.' 
### /metrics
returns prometheus-formatted metrics that can be scraped for info on deployed instance and route request durations, if
this is interesting to you.
### /public/* 
anything under /public/ is served directly as a static file output.
