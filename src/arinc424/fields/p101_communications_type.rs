use crate::enum_parseable;

enum_parseable! {
    pub enum: 3 => CommunicationsType {
        AreaControlCenter = "ACC",
        AirliftCommandPost = "ACP",
        AirToAir = "AIR",
        ApproachControl = "APP",
        ArrivalControl = "ARR",

        /// **Automatic Surface Observing System (ASOS)**
        ASOS = "ASO",

        /// **Automatic Terminal Info Service (ATIS)**
        ATIS = "ATI",

        /// **Automatic Weather Information Broadcast (AWIB)**
        AWIB = "AWI",

        /// **Automatic Weather Observing Service (AWOS)**
        AWOS = "AWO",

        /// **Aerodome Weather Information Services (AWIS)**
        AWIS = "AWS",

        ClearanceDelivery = "CLD",
        ClearancePreTaxi = "CPT",
        ControlAreaTerminal = "CTA",
        Control = "CTL",
        DepartuerControl = "DEP",

        /// **Director (Approach Control Radar)**
        Director = "DIR",

        /// **Enroute Flight Advisory Service (EFAS)**
        EFAS = "EFS",

        Emergency = "EMR",
        FlightServiceStation = "FSS",
        GroundCommOutlet = "GCO",
        GroundControl = "GND",
        GateControl = "GTE",
        HelicopterFrequency = "HEL",
        Information = "INF",
        MilitaryFrequency = "MIL",
        Multicom = "MUL",
        Operations = "OPS",

        /**
        The Comm Type PAL is used only when the frequency(s) published are used exclusively for
        the activation of airport lighting. If the pilot activation of airport lighting is
        accomplished on a frequency that is also used for voice communications, the Pilot
        Controlled Lighting parameter of the Service Indicator is used.
        */
        PilotActivatedLighting = "PAL",

        Radio = "RDO",
        Radar = "RDR",

        /// **Remote Flight Service Station (RFSS)**
        RFSS = "RFS",

        RampTaxiControl = "RMP",

        /// **Airport Radar Service Area (ARSA)**
        ARSA = "RSA",

        /// **Terminal Control Area (TCA)**
        TCA = "TCA",

        /// **Terminal Control Area (TMA)**
        TMA = "TMA",

        Terminal = "TML",

        /// **Terminal Radar Service Area (TRSA)**
        TRSA = "TRS",

        /// **Transcriber Weather Broadcast (TWEB)**
        TWEB = "TWE",

        TowerAirTrafficControl = "TWR",
        UpperAreaControl = "UAC",
        Unicom = "UNI",
        Volmet = "VOL",

        None = "   ",
    }
}
