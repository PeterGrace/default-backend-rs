# default-backend-rs

## Why
I decided to write default-backend-rs because I wanted to create a themed error page for my ingress controllers.
There was an example in the ingress-nginx repository but it was written in Go, and I'm not as familiar with Go.

I wanted to write a default-backend that supported a templating language and that also was able to emit metrics to
prometheus.

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
to be the my_deployed_dbrs service **IN THE SAME NAMESPACE AS THE INGRESS.**  The custom-http-errors would enable the
nginx-ingress improved error passthrough information as specified [in the nginx-ingress custom errors handler docs.](http://kubernetes.github.io/ingress-nginx/user-guide/custom-errors/)

## How to customize the error webpages
The container, when it starts, checks for the existence of certain files under /app/templates and /app/public.  If those
files do not exist, it will copy the default files into those paths.

If you create volume mounts at those two paths, you can customize the error pages to your liking.  You could also build 
a Docker image `FROM` this container to customize the default files for your usage; the files live in templates/ and 
public/ in this repository. 

## Special routes
There are some special routes available:
### /health
returns a 200 with json status 'ok.' 
### /metrics
returns prometheus-formatted metrics that can be scraped for info on deployed instance and route request durations, if
this is interesting to you.
### /public/* 
anything under /public/ is served directly as a static file output.