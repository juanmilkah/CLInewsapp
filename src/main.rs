use std::error::Error;

use clinews::{get_articles, Articles,};
use colour::{dark_blue, yellow};
use dotenv::dotenv;


fn render_articles(articles: &Articles){
    for a in &articles.articles{
        dark_blue!("> {}\n\n",a.title);
        yellow!("> {}\n\n",a.url);
    }
}

fn main()-> Result<(),Box<dyn Error>> {
    
    dotenv()?;


  let apikey= std::env::var("API_KEY")?;
    
    let url = "https://newsapi.org/v2/everything?q=apple&from=2024-10-15&to=2024-10-15&sortBy=popularity&apiKey=";
    let url = format!("{}{}" , url, apikey);
    let articles = get_articles(&url)?;
    render_articles(&articles);
   Ok(())
}

