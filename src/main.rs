use std::time::Duration;

use eyre::WrapErr as _;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::windows::named_pipe::{ClientOptions, PipeMode, ServerOptions};
use windows::Win32::Foundation::ERROR_PIPE_BUSY;

const PIPE_PATH: &str = r"\\.\pipe\test-pipe";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut server = ServerOptions::new()
        .first_pipe_instance(true)
        .pipe_mode(PipeMode::Message)
        .create(PIPE_PATH)?;

    let server_task = tokio::task::spawn(async move {
        println!("waiting for connection...");
        server.connect().await?;
        for i in 0..25 {
            // let message = format!("{i}: This message is kinda long");
            let message = format!("{i}: {LOREM_IPSUM}");
            println!("writing...");
            server.write_all(message.as_bytes()).await?;
            server.flush().await?;
            println!("wrote message {i}");
        }

        Result::<_, eyre::Error>::Ok(())
    });

    let client_task = tokio::task::spawn(async {
        let mut client = loop {
            let client = ClientOptions::new()
                .pipe_mode(PipeMode::Message)
                .open(PIPE_PATH);
            match client {
                Ok(client) => break client,
                Err(error) if error.raw_os_error() == Some(ERROR_PIPE_BUSY.0 as i32) => {
                    // The pipe is busy, so try connecting again after a short delay
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
                Err(error) => {
                    return Err(error).wrap_err("failed to connect to pipe");
                }
            }
        };

        let mut buffer = vec![0; 16];
        loop {
            // buffer.fill(0);
            // println!("reading...");
            // let result = client.read(&mut buffer).await;
            buffer.clear();
            let result = client.read_to_end(&mut buffer).await;
            println!("read result: {result:?}");
            println!("buffer: {}", bstr::BStr::new(&buffer));
            println!("-----------");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Result::<_, eyre::Error>::Ok(())
    });

    let (client_result, server_result) = tokio::try_join!(client_task, server_task)?;
    client_result?;
    server_result?;

    Ok(())
}

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla consectetur imperdiet metus, quis rhoncus augue ultricies a. Sed sodales quam in orci pharetra ullamcorper et sed enim. Quisque eget mauris et dui consectetur placerat. Aenean laoreet iaculis orci finibus imperdiet. Nullam placerat fringilla eleifend. Donec viverra auctor lacus, sed tincidunt mi consequat eget. Suspendisse ullamcorper dapibus ullamcorper. Fusce eget pulvinar mi. Integer diam odio, feugiat ac mattis ac, hendrerit eu magna. Nam sodales in ligula non aliquet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam pulvinar sollicitudin lacus sit amet interdum. Phasellus sapien nisi, eleifend viverra massa a, faucibus aliquet dui. In sollicitudin, nulla eget lacinia malesuada, neque ipsum dictum nisi, quis bibendum lorem lorem in massa. Duis vitae dolor magna.

Maecenas tempor nunc vitae quam vestibulum bibendum. Cras ut odio nunc. Maecenas placerat metus ac vehicula imperdiet. Nunc condimentum sed magna non molestie. Nunc a pretium nibh, sit amet rutrum quam. Vestibulum nec placerat ex, quis hendrerit leo. Donec maximus eleifend massa ac elementum. Proin non ligula ligula. Integer sit amet dui rhoncus, pretium urna posuere, molestie arcu. Nunc nec rutrum lacus. Nulla sagittis ornare arcu, ut tristique leo ultricies ut. Nam volutpat nunc nec neque imperdiet, eu pharetra ante vulputate. Phasellus rutrum porta nunc convallis tristique. Nulla ante neque, ullamcorper a tortor vulputate, hendrerit auctor erat. Vestibulum a augue mi.

Aliquam mollis tincidunt ante. Etiam ac dictum enim, sed euismod magna. Nulla tincidunt laoreet urna, eu suscipit elit ullamcorper in. Donec sed leo sem. Ut consequat suscipit malesuada. Phasellus consectetur mi ut velit suscipit dignissim eget at lectus. Nulla semper metus sit amet nunc ultricies tincidunt. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Sed cursus porta quam, dignissim ornare metus scelerisque a. Suspendisse interdum nunc et turpis rhoncus porttitor. Aliquam maximus arcu imperdiet, venenatis justo vitae, convallis diam. Mauris efficitur felis sed hendrerit lobortis.

Sed ultricies interdum orci eu auctor. Donec vitae arcu vel nisl porttitor pellentesque. Suspendisse ac vehicula ante. Aliquam euismod augue et enim iaculis, ac varius felis faucibus. Cras tortor dui, pharetra sit amet ex vitae, auctor bibendum mi. Vestibulum vel velit nisl. Morbi dictum hendrerit enim at faucibus. Proin odio nulla, posuere ut scelerisque a, tristique quis arcu. Integer mauris est, maximus quis lectus ac, tincidunt bibendum odio. Vivamus nec tempus felis.

Sed sagittis elit ipsum, ac volutpat nisl vehicula a. Vestibulum non congue tortor. Duis non cursus risus, eu scelerisque turpis. Vestibulum sit amet libero imperdiet lacus semper auctor. Integer pharetra, ligula quis mollis volutpat, risus neque ullamcorper diam, pharetra viverra mauris libero at tellus. Nullam sed venenatis nulla, eget aliquam metus. Sed efficitur magna ipsum, non sodales nunc hendrerit id.

Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas tincidunt in nulla vel pellentesque. Nullam quis porta ligula. Cras porttitor purus purus, sed sollicitudin eros luctus eget. Ut vitae ullamcorper augue. Nunc sit amet turpis nunc. Quisque pulvinar hendrerit magna, ac laoreet ipsum cursus id. Vivamus luctus mauris vitae viverra aliquet. Maecenas varius sem ex, sit amet dignissim ante vehicula a. Nam vel justo eget velit dapibus porta non eu leo. Ut lacinia volutpat massa, varius malesuada nisl dictum quis. Suspendisse gravida mi quis ex bibendum imperdiet. Sed semper facilisis tristique. Nullam posuere, ipsum sed varius aliquet, nisi tortor fringilla libero, quis aliquam mi felis in purus. Nam elit mauris, hendrerit eu facilisis nec, porta vel felis. Duis tristique diam felis, posuere maximus nunc varius eu.

Quisque nec elit erat. Nam ornare felis eu lacinia mollis. Nam urna orci, tempus in risus in, porta ornare nibh. Pellentesque a viverra dui. Vivamus nibh ligula, ultrices sed faucibus at, hendrerit vitae orci. Vivamus lacinia libero in augue convallis hendrerit. Donec tempus luctus augue at rhoncus. Ut auctor augue augue, id efficitur ex tincidunt in. Aenean dignissim, diam vitae maximus tempus, ipsum nunc posuere nibh, at condimentum mi felis ut orci. Morbi facilisis metus urna, id congue sapien dignissim a. Ut quis aliquam velit. Nulla eu accumsan massa, nec condimentum nunc. Proin ac ullamcorper enim, id vehicula nibh. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.

Donec justo odio, faucibus in pretium vitae, euismod sit amet enim. Aenean ornare sed diam ullamcorper aliquet. Integer in felis ullamcorper, porta erat id, maximus augue. Ut porta at sapien sit amet rutrum. Mauris ac augue eu lacus cursus tincidunt. Sed luctus blandit purus varius mollis. Nulla facilisis viverra neque, gravida ultricies erat aliquet et non.";
