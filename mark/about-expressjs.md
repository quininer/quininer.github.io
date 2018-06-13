# 关于expressjs的一个小坑
## *Tue Dec  9 01:07:53 2014*

都知道在PHP里，`/?key[arr]=value` 会被解析为

    array(1) { ["key"]=> array(1) { ["arr"]=> string(5) "value" } }

却一直没注意到expressjs里也会如此，

    // GET /shoes?order=desc&shoe[color]=blue&shoe[type]=converse
    req.query.order
    // => "desc"

    req.query.shoe.color
    // => "blue"

    req.query.shoe.type
    // => "converse"

而在[req.query](http://expressjs.com/4x/api.html#req.query)里仅仅是这样不太显眼的一笔带过，似乎嵌套解析已成标配。
但是不得不说的是嵌套解析至少在Node.js里很危险。

例如

    app.get('/', function(req, res){
        sql.find({_id:req.query.id})
        .exec(function(err, data){
            res.json({data:data});
        });
    });

访问`/?id[$ne]=1`则会返回不该有的数据，正如[这里](http://drops.wooyun.org/tips/3939)。

当然这不是mongodb的问题，参数的属性也可能被劫持

    app.get('/', function(req, res){
        if(req.query.name.length <= 5){
            // TODO
            res.json({'err':null, 'name':req.query.name});
        }else{
            res.json({'err':'名字过长！'});
        };
    });

使用`/?name[length]=1`即能绕过这个名字长度的限制。

同样的利用方法在更奇怪的环境可能会造成更严重的后果，最终导致**code injection**也并非不可能。  
大概是意识到了这点，开发团队在下一个Web框架[koa](http://koajs.com/#request-query)里取消了对嵌套解析的支持。  
后续分离的[cookie-parser](https://github.com/expressjs/cookie-parser)也不支持嵌套解析。

最后，我的粗略的解决方案是

    app.get('/', function(req, res){
        var name = (typeof req.query.name == 'string')?req.query.name:undefined;
        // TODO
    });

或是使用中间件

    app.use(function(req, res, next){
        for(var i in req.query)if(typeof req.query[i] != 'string')delete req.query[i];
        for(var i in req.body)if(typeof req.body[i] != 'string')req.body[i] = JSON.stringify(req.body[i]);
        next();
    });

--------------------

**总之，务必先验证`req.query.name`，`req.body.name`以及`req.param('name')`的类型**
