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
      <h1><img src="data:image/png;base64, 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" alt="Jua">ipgiraffe.com</h1>
    </header>
    <main>
      <p>Your IP address is:</p>
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
