use crate::autoparse_struct;
use crate::enum_parseable;

autoparse_struct! {
    /**
    The "Service Indicator" field is used to further define the use of the frequency for the
    specified Communication Type (5.101).

    *This is a split field, `ServiceIndicatorEnroute` contains the equivalent for enroute
    frequencies.*
    */
    pub ServiceIndicatorAirport {
        service: ServiceIndicatorAirportService,
        frequency: ServiceIndicatorAirportFrequency,
        usage: ServiceIndicatorAirportUsage,
    }
}

autoparse_struct! {
    /**
    The "Service Indicator" field is used to further define the use of the frequency for the
    specified Communication Type (5.101).

    *This is a split field, `ServiceIndicatorAirport` contains the equivalent for airport
    frequencies.*
    */
    pub ServiceIndicatorEnroute {
        service: ServiceIndicatorEnrouteService,
        frequency: ServiceIndicatorEnrouteFrequency,
        usage: ServiceIndicatorEnrouteUsage,
    }
}

enum_parseable! {
    pub enum ServiceIndicatorAirportService {
        /// **Airport Advisory Service (AAS)**
        AAS = "A",

        /// **Community Aerodrome Radio Station (CARS)**
        CARS = "C",

        /// **Departure Service (Other than Departure Control Unit)**
        DepartureService = "D",

        /// **Flight Information Service (FIS)**
        FIS = "F",

        /// **Initial Contact (IC)**
        IC = "I",

        /// **Arrival Service (Other than Arrival Control Unit)**
        ArrivalService = "L",

        /// **Pre-Departure Clearance (Data Link Service)**
        PreDepartureClearance = "P",

        /// **Aerodrome Flight Information Service (AFIS)**
        AFIS = "S",

        /// **Terminal Area Control (Other than dedicated Terminal Control Unit)**
        TerminalAreaControl = "T",

        None = " ",
    }
}

enum_parseable! {
    pub enum ServiceIndicatorAirportFrequency {
        /// **Aerodrome Traffic Frequency (ATF)**
        ATF = "A",

        /// **Common Traffic Advisory Frequency (CTAF)**
        CTAF = "C",

        /// **Mandatory Frequency**
        Mandatory = "M",

        /// **Air/Air**
        AirToAir = "R",

        /// **Secondary Frequency**
        Secondary = "S",

        None = " ",
    }
}

enum_parseable! {
    pub enum ServiceIndicatorAirportUsage {
        /// **Air/Ground**
        AirToGround = "A",

        /// **VHF Direction Finding Service (VDF)**
        VDF = "D",

        /// **Remote Communications Air to Ground (RCAG)**
        RCAG = "G",

        /// **Language other than English**
        NonEnglish = "L",

        /// **Military Use Frequency**
        MilitaryUse = "M",

        /// **Pilot Controlled Light (PCL)**
        PCL = "P",

        /// **Remote Communications Outlet (RCO)**
        RCO = "R",

        None = " ",
    }
}

enum_parseable! {
    pub enum ServiceIndicatorEnrouteService {
        /// **Aeronautical Enroute Information Service (AEIS)**
        AEIS = "A",

        /// **Flight Information Service (FIS)**
        FIS = "F",

        None = " ",
    }
}

enum_parseable! {
    pub enum ServiceIndicatorEnrouteFrequency {
        /// **Air/Ground**
        AirToGround = "A",

        /// **Discrete Frequency**
        Discrete = "D",

        /// **Air/Air**
        AirToAir = "R",

        /// **Mandatory Frequency**
        Mandatory = "M",

        /// **Secondary Frequency**
        Secondary = "S",

        None = " ",
    }
}

enum_parseable! {
    pub enum ServiceIndicatorEnrouteUsage {
        /// **VHF Direction Finding Service (VDF)**
        VDF = "D",

        /// **Remote Communications Air to Ground (RCAG)**
        RCAG = "G",

        /// **Language other than English**
        NonEnglish = "L",

        /// **Military Use Frequency**
        MilitaryUse = "M",

        /// **Remote Communications Outlet (RCO)**
        RCO = "R",

        None = " ",
    }
}
