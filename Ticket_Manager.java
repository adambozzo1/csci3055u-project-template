/**
 * Ticket Manager class used to sell, buy and manage all the tickets
 * @author Adam Bozzo
 * @version 1.0
 * @since  03/10/2019
 */

import java.io.*;
import java.util.ArrayList;

public class Ticket_Manager {
    ArrayList<Ticket>ticketList;
    /**
     * Reads and parses the ticket file and creates tickets based on information read
     * @param ticketFile is the file containing all the ticket information
     * @return void
     */
    void readTicketFile(File ticketFile){
        try{
            BufferedReader reader = new BufferedReader(new FileReader(ticketFile));
            String line = reader.readLine();
            ticketList = new ArrayList<>();
            while(line != null){
                Ticket tempTicket = new Ticket(line.substring(0,19), line.substring(20,34),
                        Integer.parseInt(line.substring(35,38)), Float.parseFloat(line.substring(39,45)));
                ticketList.add(tempTicket);
                line = reader.readLine();
            }
            reader.close();
        }catch(Exception e){
            System.err.println("Error reading ticket file!");
            Logger.addError("Error reading ticket file!");
        }
    }
    /**
     * Creates a new ticket to be sold
     * @param sTicket is the transaction that contains all information for new ticket to be sold
     * @return void
     */
    void sellTicket(Transaction sTicket){
        boolean isExist = false;
        for(Ticket temp: ticketList){
            if(sTicket.seller.equals(temp.sellerName) && sTicket.event.equals(temp.eventName)){
                isExist = true;
                break;
            }
        }
        if(!isExist){
            Ticket newTicket = new Ticket(sTicket.event, sTicket.seller, sTicket.tickets, sTicket.ppp);
            ticketList.add(newTicket);
        }else{
            System.err.println("Ticket already exists!");
            Logger.addError("Ticket already exists!");
        }
    }
    /**
     * Buys a ticket from a seller and removes ticket quantity from seller based on how many tickets being bought
     * @param bTicket is the transaction that contains all information on ticket being bought
     * @return void
     */
    void buyTicket(Transaction bTicket){
        if(0 < bTicket.tickets & bTicket.tickets <= 4){
            int index = 0;
            for(Ticket temp: ticketList){
                if(temp.eventName.equals(bTicket.event) && temp.sellerName.equals(bTicket.seller)){
                    if(temp.numTicket >= bTicket.tickets){
                        ticketList.get(index).numTicket -= bTicket.tickets;
                        break;
                    }else{
                        System.err.println("Not enough tickets left to sell!");
                        Logger.addError("Not enough tickets left to sell!");
                    }
                }
                index++;
            }
        }else{
            System.err.println("Too many tickets trying to be purchased!");
            Logger.addError("Too many tickets trying to be purchased!");
        }
    }
    //method to write the newly updated ticket file
    void writeTicketFile(){
        File file = new File("ticketfile.txt");
        FileWriter fr = null;
        try {
            fr = new FileWriter(file, true);
            for(Ticket temp : ticketList){
                String numTicket = "" + temp.numTicket;
                String ticketPrice = "" + temp.ticketPrice;
                //if statements for fixing format of number of tickets
                if(numTicket.length() == 2){
                    numTicket = "0" + numTicket;
                }else if(numTicket.length() == 1) {
                    numTicket = "00" + numTicket;
                }
                //if statements to fix format of ticket price
                if(ticketPrice.charAt(ticketPrice.length()-2) == '.'){
                    if(ticketPrice.length() == 3){
                        ticketPrice = "00" + ticketPrice + "0";
                    }else if(ticketPrice.length() == 4){
                        ticketPrice = "0" + ticketPrice + "0";
                    }else if(ticketPrice.length() == 5) {
                        ticketPrice = ticketPrice + "0";
                    }
                }else{
                    if(ticketPrice.length() == 4){
                        ticketPrice = "00" + ticketPrice ;
                    }else if(ticketPrice.length() ==5){
                        ticketPrice = "0" + ticketPrice;
                    }
                }

                fr.write("\n" + temp.eventName + " " +  temp.sellerName + " " + numTicket + " " + ticketPrice);
            }
            fr.close();
        } catch (IOException e) {
            System.err.println("Error writing to ticket file");
        }
    }
}
