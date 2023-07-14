use indoc::indoc;

pub const NULL_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

pub fn get_banner() -> &'static str {
    let banner = indoc! {
r#"

$$$$$$$$\  $$$$$$\  $$$$$$$$\  $$$$$$\         $$$$$$$\   $$$$$$\                       
$$  _____|$$  __$$\ $$  _____|$$  __$$\        $$  __$$\ $$  __$$\                      
$$ |      $$ /  \__|$$ |      $$ /  \__|       $$ |  $$ |$$ /  \__|                     
$$$$$\    $$ |      $$$$$\    \$$$$$$\ $$$$$$\ $$$$$$$  |\$$$$$$\                       
$$  __|   $$ |      $$  __|    \____$$\\______|$$  __$$<  \____$$\                      
$$ |      $$ |  $$\ $$ |      $$\   $$ |       $$ |  $$ |$$\   $$ |                     
$$ |      \$$$$$$  |$$ |      \$$$$$$  |       $$ |  $$ |\$$$$$$  |                     
\__|       \______/ \__|       \______/        \__|  \__| \______/                      
                                                                                        
                                                                                        
                                                                                        
$$$$$$$\ $$\     $$\        $$$$$$\  $$\   $$\ $$$$$$$$\ $$$$$$$\ $$$$$$$$\ $$\     $$\ 
$$  __$$\\$$\   $$  |      $$  __$$\ $$ |  $$ |$$  _____|$$  __$$\\__$$  __|\$$\   $$  |
$$ |  $$ |\$$\ $$  /       $$ /  $$ |$$ |  $$ |$$ |      $$ |  $$ |  $$ |    \$$\ $$  / 
$$$$$$$\ | \$$$$  /        $$ |  $$ |$$ |  $$ |$$$$$\    $$$$$$$  |  $$ |     \$$$$  /  
$$  __$$\   \$$  /         $$ |  $$ |$$ |  $$ |$$  __|   $$  __$$<   $$ |      \$$  /   
$$ |  $$ |   $$ |          $$ $$\$$ |$$ |  $$ |$$ |      $$ |  $$ |  $$ |       $$ |    
$$$$$$$  |   $$ |          \$$$$$$ / \$$$$$$  |$$$$$$$$\ $$ |  $$ |  $$ |       $$ |    
\_______/    \__|           \___$$$\  \______/ \________|\__|  \__|  \__|       \__|  
"#};
    banner
}