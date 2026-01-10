// src/sender.rs

use crate::{RoomMetrics, debug, error, info, init_logger, trace, warn};
use bincode;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub struct MetricsSender {
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(bind_addr)?;

        // Инициализируем логирование, если фича включена
        init_logger();

        info!("MetricsSender создан на адресе {}", bind_addr);

        Ok(Self { socket })
    }


    // Метод отправки сообщений в сокет
    pub fn send_to(
        &self,
        metrics: &RoomMetrics,
        target_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {

        debug!("Сериализация метрик: {:?}", metrics);
        let encoded = bincode::serialize(metrics)?;
        
        debug!("Отправка {} байт на {}", encoded.len(), target_addr);
        // self.socket.send_to(&encoded, target_addr)?;
        let sent_bytes = self.socket.send_to(&encoded, target_addr)?;

        trace!("Успешно отправлено {} байт", sent_bytes);
        Ok(())
    }

    // Метод для запуска цикла постоянной отправки метрик
    pub fn start_broadcasting(
        
        self,
        target_addr: String,
        interval_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {


        // println!(
        //     "Имитатор датчиков запущен. Отправка на {} каждые {}ms",
        //     target_addr, interval_ms
        // );

        info!(
            "Запуск трансляции метрик на {} каждые {} мс",
            target_addr, interval_ms
        );



        // // Информация о включённых фичах
        // #[cfg(feature = "random")]
        // println!("✅ Фича 'random' активна - используется rand для генерации данных");
       
        // Информация о включённых фичах
        #[cfg(feature = "random")]
        info!("Фича 'random' активна - используется rand для генерации данных");

        // #[cfg(not(feature = "random"))]
        // println!("ℹ️  Фича 'random' отключена - используется детерминистическая генерация");

        #[cfg(not(feature = "random"))]
        warn!("Фича 'random' отключена - используется детерминистическая генерация");
        
        let mut counter = 0;
        loop {
            counter += 1;
            debug!("Генерация метрик #{}", counter);
            let metrics = RoomMetrics::random();
            trace!("Сгенерированные метрики: {:?}", metrics);

            match self.send_to(&metrics, &target_addr) {
                Ok(()) => {
                    info!(
                        "[{}] Отправлено: {:.1}C, {:.1}% влажности, давление: {:.1}hPa, дверь: {}",
                        metrics.formatted_time(),
                        metrics.temperature,
                        metrics.humidity,
                        metrics.pressure,
                        if metrics.door_open {
                            "открыта"
                        } else {
                            "закрыта"
                        },
                    );

                    // // Демонстрация фичи sqlite
                    // #[cfg(feature = "sqlite")]
                    // {
                    //     println!("   💾 SQL: {}", metrics.to_sql());
                    // }
                    // Демонстрация фичи sqlite
                    #[cfg(feature = "sqlite")]
                    {
                        debug!("SQL-запрос: {}", metrics.to_sql());
                    }
                }
                Err(e) => {
                    eprintln!("Ошибка отправки: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(interval_ms));
        }
    }
}