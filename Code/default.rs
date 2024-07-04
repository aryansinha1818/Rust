pub  struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String,
}

// Now let's implement the summary traits for newsarticle type
impl Summary for NewsArticle{
      
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}

//Default implementation
pub trait Summary{
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

fn main(){
    let tweet = Tweet{
        username: String::from("@johndoe"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };


    let article = NewsArticle{
    author: String::from("John Doe"),
    headline: String::from("The Sky is falling!"),
    content: String::from("The sky is not actually falling.")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}