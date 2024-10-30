use teloxide::prelude::*; // Importar herramientas principales de Teloxide
use teloxide::types::Message; // Importar el tipo de mensaje
use dotenv::dotenv; // Importar dotenv para cargar las variables de entorno
use std::env; // Importar env para trabajar con variables de entorno
use log; // Importar log para registrar mensajes en la consola

// Esta es la función principal que se ejecuta primero
#[tokio::main]
async fn main() {
    // Cargar las variables de entorno desde el archivo `.env`
    dotenv().ok(); // Esto carga las configuraciones del archivo .env, donde guardamos información importante

    // Obtener el token del bot desde la variable de entorno
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN no encontrado. Asegúrate de configurarlo en el archivo .env");
    // Aquí tomamos el valor del token del bot, que es como la contraseña para que nuestro bot funcione.
    // Si no encuentra el token, mostrará un error diciendo que no se encontró.

    // Inicializar el bot con el token
    let bot = Bot::new(bot_token).auto_send();
    // Creamos un nuevo bot usando el token que obtuvimos. El `auto_send` hace que el bot envíe mensajes automáticamente sin que tengamos que preocuparnos por algunos detalles técnicos.

    // Registrar un logger para tener información adicional en la consola
    pretty_env_logger::init();
    log::info!("Bot iniciado...");
    // El logger es como un narrador. Nos dice qué está pasando mientras el bot está funcionando.
    // Aquí lo configuramos para que nos informe cuando el bot se inicia.

    // Crear el Dispatcher para manejar mensajes
    let mut dispatcher = Dispatcher::builder(bot, dptree::entry()
        .branch(Update::filter_message().endpoint(responder_a_mensaje)))
        .build();
    // El Dispatcher es como el coordinador del bot. Se encarga de recibir los mensajes y decidir qué hacer con ellos.
    // Aquí le decimos que cuando reciba un mensaje, use la función `responder_a_mensaje` para manejarlo.

    // Iniciar el dispatcher para escuchar mensajes
    dispatcher.dispatch().await;
    // Esta línea hace que el bot empiece a escuchar los mensajes que llegan y actúe según las reglas que le dimos.
}

// Función manejadora del mensaje
async fn responder_a_mensaje(
    bot: AutoSend<Bot>, // Este es el bot que enviará la respuesta
    mensaje: Message,   // Este es el mensaje que recibió el bot
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Esta función se llama cada vez que el bot recibe un mensaje de texto
    if let Some(texto) = mensaje.text() {
        // Si el mensaje tiene algún texto (no es una imagen, sticker, etc.), entonces ejecutamos lo siguiente
        
        // Responder al mensaje recibido
        bot.send_message(mensaje.chat.id, format!("Dijiste: {}", texto)).await?;
        // Aquí el bot responde al chat diciendo "Dijiste: " seguido del texto que envió el usuario.
    }
    Ok(()) // Si todo salió bien, devolvemos Ok(). Esto significa que la función terminó sin errores.
}

// Resumen:
// 1. Cargamos las configuraciones del archivo `.env`.
// 2. Creamos un bot usando el token del archivo `.env`.
// 3. Configuramos un "Dispatcher" que se encargará de manejar los mensajes.
// 4. Definimos una función (`responder_a_mensaje`) que simplemente responde a cada mensaje de texto que reciba el bot.
// 5. Iniciamos el Dispatcher para empezar a recibir mensajes y manejar todo automáticamente.
