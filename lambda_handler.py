import json

def lambda_handler(event, context):
    if 'xForwardedFor' in event and event['xForwardedFor']:
        client_ip = event['xForwardedFor'].split(',')[0]
    elif 'sourceIp' in event:
        client_ip = event['sourceIp']
    else:
        client_ip = "IP Address Not Found"
    html_response = """
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>ipgiraffe.com</title>
    <style>
      @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@300;400&display=swap');
      body {
        background-color: #f0e5c9; /* A light tan base color */
        background-image: 
          radial-gradient(circle farthest-side, #a06e3b 50%, transparent 55%),
          radial-gradient(circle farthest-side, #a06e3b 50%, transparent 55%);    
        background-size: 80px 80px; /* Adjust the size of spots */
        background-position: 0 0, 40px 40px; /* Offset spots for a natural look */
	    font-family: Arial, Helvetica, sans-serif;
	    font-size: 16px;
      }
      header {
        background: #f0e5c9;
        color: #a06e3b;
        border: 4px solid #a06e3b;
        border-radius: 10px;
        margin: 10px;
        padding: 5px;
        /* Centering the text */
        text-align: center;     /* Horizontal centering */
        display: flex;          /* Enable flexbox for vertical centering */
        align-items: center;    /* Center vertically within the flex container */
        justify-content: center; /* Center horizontally if there's extra space */
      }
      header h1 {
        font-family: 'Poppins', Arial, Helvetica, sans-serif;
      }
      main {
        background: #f0e5c9;
        color: #784820;
        border: 4px double #a06e3b;
        font-size: 1.2em;
        line-height: 1.6; 
        border-radius: 10px;
        margin: 10px;
        padding: 20px;
        /* Centering the text */
        text-align: center;     /* Horizontal centering */
        align-items: center;    /* Center vertically within the flex container */
        justify-content: center; /* Center horizontally if there's extra space */
      }
      main h2 {
        margin-bottom: 15px; /* Add spacing below the IP */
	    font-family: 'Poppins', Arial, Helvetica, sans-serif; 
      }
	  .copyright {
        font-size: 0.7em; /* Smaller font size */
      }
    </style>
  </head>
  <body>
    <header>
      <h1>ipgiraffe.com</h1>
    </header>
    <main>
      <p>Your computer&apos;s IP address is:</p>
      <h2>""" + client_ip + """</h2>
	  <p class="copyright">&copy; 2024 ipgiraffe.com is hosted on AWS Lambda</p>
	</main>
      <!-- 
                  n          n                               
                .'_`=      ='_e.                                
              .e/              \e.                               
           .-e (                ) e-.                            
         .e . e)                (e ,e`.                          
      ,-<.--'\|>                /|/`--.>-,                       
        |\   ,|                / |    /|      a:f    
	  https://www.wildnatureinstitute.org/save-the-giraffe.html
    -->
  </body>
</html>
    """

    return {
        'statusCode': 200,
        'body': html_response,
        'headers': {
            'Content-Type': 'text/html'
        }
    }
