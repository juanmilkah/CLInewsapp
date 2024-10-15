use std::error::Error;
use serde::Deserialize;
use ureq;
use colour::{dark_blue, yellow};

#[derive(Deserialize,Debug)]
struct Articles{
    articles: Vec<Article>
}
#[derive(Deserialize,Debug)]
struct Article{
    title:String,
    url: String
}

fn get_articles(url:&str)->Result<Articles,Box<dyn Error>>{
    let response = ureq::get(url).call()?.into_string()?;
    let articles:Articles = serde_json::from_str(&response)?;
    Ok(articles)
    
}
fn render_articles(articles: &Articles){
    for a in &articles.articles{
        dark_blue!("> {}\n\n",a.title);
        yellow!("> {}\n\n",a.url);
    }
}

fn main()-> Result<(),Box<dyn Error>> {
    let url = "https://newsapi.org/v2/everything?q=tesla&from=2024-09-15&sortBy=publishedAt&apiKey=6892d0355c8e48d29cf54eea94e9cfcd";
    let articles = get_articles(url)?;
    render_articles(&articles);
   Ok(())
}

