pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary{
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle{
      
}


impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}


// pub fn notify(item: &impl Summary){
//     println!("Breaking News! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}

//More than one parameter
pub fn name1(item1: &impl Summary, item2: &impl Summary){
    //

}

//Using trait bound to make the code look clean
pub fn name1<T: Summary>(item1: &T, item2: &T){
    //
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
    notify(&article);
}